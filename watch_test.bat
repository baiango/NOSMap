:loop
	cls
	tcc src\main.c -o main.exe -DTESTS && main.exe
	timeout /t 5
goto loop
