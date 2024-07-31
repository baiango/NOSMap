:loop
	cls
	gcc src\*.c -o main.exe -g -mavx2 -DTESTS && gdb --ex=run --ex=bt --batch main.exe
	timeout /t 5
goto loop
