#!/home/juancsu/miniforge3/bin/python

import inspect
from rocker import git_extension

extension_classes = []
for name, obj in inspect.getmembers(git_extension):
  if inspect.isclass(obj) and [b.__name__ for b in obj.__bases__][0] == 'RockerExtension':
    print("Extension Class: ", name, " Inherits: ", [b.__name__ for b in obj.__bases__])
    extension_classes.append(obj)