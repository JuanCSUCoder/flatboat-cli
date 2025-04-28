import inspect
import typing
import rocker
import os

# Default Extensions Imports
import rocker.core
import rocker.extensions
import rocker.git_extension
import rocker.nvidia_extension
import rocker.os_detector
import rocker.rmw_extension
import rocker.ssh_extension
import rocker.volume_extension

## BEGIN Open Robotics Topological Extension Sorter - APACHE 2.0 ##

def sort_extensions(extensions, cli_args=""):
  def topological_sort(source: typing.Dict[str, typing.Set[str]]):
    """Perform a topological sort on names and dependencies and returns the sorted list of names."""
    names = set(source.keys())
    # prune optional dependencies if they are not present (at this point the required check has already occurred)
    pending = [(name, dependencies.intersection(names)) for name, dependencies in source.items()]
    emitted = []
    while pending:
      next_pending = []
      next_emitted = []
      for entry in pending:
        name, deps = entry
        deps.difference_update(emitted)  # remove dependencies already emitted
        if deps:  # still has dependencies? recheck during next pass
          next_pending.append(entry)
        else:  # no more dependencies? time to emit
          yield name
          next_emitted.append(name)  # remember what was emitted for difference_update()
        #end if
      #end for
      if not next_emitted:
        raise Exception("Cyclic dependancy detected: %r" % (next_pending,))
      #end if
      pending = next_pending
      emitted = next_emitted
    #end while
  #end def

  extension_graph = {name: cls.invoke_after(cli_args) for name, cls in sorted(extensions.items())}
  active_extension_list = [extensions[name] for name in topological_sort(extension_graph)]
  return active_extension_list
#end def

## END Open Robotics Topological Extension Sorter - APACHE 2.0 ##

def get_extensions(extensions_modules: list, args_dict: dict):
  # 1. Get Default Extensions Modules
  extension_mods = [rocker.extensions, rocker.git_extension, rocker.nvidia_extension, rocker.rmw_extension, rocker.ssh_extension, rocker.volume_extension] + extensions_modules

  # 2. Get Filtered Extensions Objects
  print("##### DETECTING EXTENSIONS #####")
  extensions_dict = {}
  for extension_mod in extension_mods:
    for name, cls in inspect.getmembers(extension_mod):
      if inspect.isclass(cls) and [b.__name__ for b in cls.__bases__][0] == 'RockerExtension' and cls.check_args_for_activation(args_dict):
        print("Extension Class: ", name, " Inherits: ", [b.__name__ for b in cls.__bases__])
        extensions_dict[name] = cls()
      #end if
    #end for
  #end for

  # 4. Sort Extensions
  return sort_extensions(extensions_dict, cli_args=args_dict)
#end def

def generate_dockerfile(extensions: list[rocker.core.RockerExtension], args_dict: dict):
  base_image = args_dict['base_image']

  dockerfile_str = ''
  # 1. Generate Preamble snippets
  for el in extensions:
    dockerfile_str += '# Preamble from extension [%s]\n' % el.get_name()
    dockerfile_str += el.get_preamble(args_dict) + '\n'
  #end for

  # 2. Start from base_image
  dockerfile_str += '\nFROM %s\n' % base_image

  # 3. Generate ROOT snippets
  dockerfile_str += 'USER root\n'
  for el in extensions:
    dockerfile_str += '# Snippet from extension [%s]\n' % el.get_name()
    dockerfile_str += el.get_snippet(args_dict) + '\n'
  #end for


  # 4. Set USER if user extension activated
  if 'user' in args_dict and args_dict['user']:
    if 'user_override_name' in args_dict and args_dict['user_override_name']:
      username = args_dict['user_override_name']
    else:
      username = "flatboat"
    #end if
    dockerfile_str += f'USER {username}\n'
  #end if

  # 5. Generate USER snippets
  for el in extensions:
      dockerfile_str += '# User Snippet from extension [%s]\n' % el.get_name()
      dockerfile_str += el.get_user_snippet(args_dict) + '\n'
  return dockerfile_str
#end def

def generate_parameters(extensions: list[rocker.core.RockerExtension], args_dict: dict):
  docker_args = ''

  for e in extensions:
    docker_args += e.get_docker_args(args_dict)
  #end for

  return docker_args.split()
#end def

def setup_environment(extensions_names: list[str], args_dict: dict) -> list[rocker.core.RockerExtension]:
  """
  Setups environment for all extensions, and returns the configurated extensions as a list.
  """
  # 1. Get the active extensions
  extensions_modules = flatboat_ext_loader(extensions_names)
  extensions: list[rocker.core.RockerExtension] = get_extensions(extensions_modules, args_dict)

  # 2. Configure all preconditions on the environment
  for e in extensions:
    e.precondition_environment(args_dict)
  #end for

  # 3. Validate the environment
  for e in extensions:
    e.validate_environment(args_dict)
  #end for

  return extensions
#end def

def flatboat_ext_loader(extensions_names: list[str]) -> list[rocker.core.RockerExtension]:
  """
  Loads flatboat rocker extensions using it's names
  """
  # TODO: Implement
  return []
#end def

def get_rocker_config(additional_extensions: list[str], args: dict) -> tuple[str, str]: 
  # 1. Setup Rocker Environment and Extensions
  extensions = setup_environment(additional_extensions, args)

  # 2. Generate Dockerfile
  df = generate_dockerfile(extensions, args)
  print(" ==================================== DOCKERFILE ===================================")
  # 3. Generate Parameters
  print(" ======================================= ARGS ======================================")
  args = generate_parameters(extensions, args)
  
  return (df, args)
#end def

if __name__ == "__main__":
  ext_mods = []
  args = {
    'base_image': 'ubuntu:22.04',

    'x11': True,

    'nvidia': True,
    'cuda': True,

    'git': True,
    'user': True,
    'network': 'host',
    'privileged': True,
    'pulse': True,
  }

  (df, args) = get_rocker_config(ext_mods, args)
  print(df)
  print(args)
  input(' ')
#end if
