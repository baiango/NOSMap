:loop
	cls
	gcc src\main.c -o main.exe -mavx2 -DTESTS && main.exe
	timeout /t 5
goto loop
