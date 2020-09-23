#!/usr/bin/python
import os
import sys

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

# MAKE SURE that terser & csso is installed locally
os.getcwd()

if len(sys.argv) > 1 and ".js" in sys.argv[1]:
    print("\ntrying to minify " + sys.argv[1] + " ...")
    os.system("exec ../../scripts/node_modules/.bin/terser " + sys.argv[1] + " -c -m --source-map \"root='https://grimstack.io/js/',url='" + sys.argv[1][:-3] + ".min.js.map'\" -o " + sys.argv[1][:-3] + ".min.js")
    
    print("done minifying, did it work? ")
else:
    print("\nno arg, minifying everything...")
    for root, dirs, files in os.walk("./"):    
        for file in files:
            if ".js" in file and ".min.js" not in file:
                print(file)
                os.system("exec ../../scripts/node_modules/.bin/terser " + file + " -c -m --source-map -o " + file[:-3] + ".min.js")

    print("done, all is minified!")