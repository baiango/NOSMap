:loop
	cls
	gcc src\*.c -o main.exe -g -mavx2 && main.exe
	timeout /t 5
goto loop
