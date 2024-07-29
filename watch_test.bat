:loop
	cls
	gcc src\*.c -o main.exe -g -mavx2 -DTESTS && main.exe
	timeout /t 5
goto loop
