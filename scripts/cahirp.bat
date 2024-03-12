@echo off
call variables.cmd

tw3-cahirp build  --game "%gamePath%" --without-mods --recipes "%modpath%/src/%modname%/cahirp" --out "%modpath%/src/%modname%/content/scripts" --clean