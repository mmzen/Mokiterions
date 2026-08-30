@echo off
python "%~dp0argvlog.py" %*
exit /b %ERRORLEVEL%
