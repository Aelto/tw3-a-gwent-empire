:: bundles the files from /wolvenkit-project for the DLC

@echo off

call variables.cmd

cd %modkitpath%

rmdir "%modpath%\wolvenkit-project\packed\content\" /s /q
mkdir "%modpath%\wolvenkit-project\packed\content\"

call wcc_lite.exe pack -dir=%modpath%\wolvenkit-project\files\mod\cooked\ -outdir=%modpath%\wolvenkit-project\packed\content\
call wcc_lite.exe metadatastore -path=%modpath%\wolvenkit-project\packed\content\

cd %modpath%\scripts