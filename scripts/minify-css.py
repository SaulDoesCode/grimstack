#!/usr/bin/python
import os
import sys
import subprocess


found_syncthing = False
try:
    # don't mess up syncthing syncing with minification seriously
    output = subprocess.check_output(['pgrep', 'syncthing'])
    if len(output) > 0:
        print('found syncthing running, so won\'t minify')
        found_syncthing = True
except:
    pass

if found_syncthing:
    sys.exit(0)

# MAKE SURE that csso & terser is installed locally
os.getcwd()

if len(sys.argv) > 1 and ".css" in sys.argv[1]:
    print("\ntrying to minify " + sys.argv[1] + " ...")
    os.system("exec ../../scripts/node_modules/.bin/csso " + sys.argv[1] + " -o " + sys.argv[1][:-3] + "min.css")
    
    print("done minifying, did it work? ")
else:
    print("\nno arg, minifying everything...")
    for root, dirs, files in os.walk("./"):    
        for file in files:
            if ".css" in file and ".min.css" not in file:
                print(file)
                os.system("exec ../../scripts/node_modules/.bin/csso " + file + " -o " + file[:-3] + "min.css")

    print("done, all is minified!")