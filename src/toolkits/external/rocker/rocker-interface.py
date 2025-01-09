import inspect
import rocker

# Default Extensions Imports
import rocker.extensions
import rocker.git_extension
import rocker.nvidia_extension
import rocker.os_detector
import rocker.rmw_extension
import rocker.ssh_extension
import rocker.volume_extension

# 1. Get Default Extensions Modules
extensions = [rocker.extensions, rocker.git_extension, rocker.nvidia_extension, rocker.rmw_extension, rocker.ssh_extension, rocker.volume_extension]


# 2. Get Extensions Classes
print("##### DETECTING EXTENSIONS #####")
extension_classes = []
for extension in extensions:
  for name, obj in inspect.getmembers(extension):
    if inspect.isclass(obj) and [b.__name__ for b in obj.__bases__][0] == 'RockerExtension':
      print("Extension Class: ", name, " Inherits: ", [b.__name__ for b in obj.__bases__])
      extension_classes.append(obj)
    #end if
  #end for
#end for

def generate_dockerfile():
  pass

def generate_parameters():
  pass