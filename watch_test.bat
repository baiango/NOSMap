:loop
	cls
	g++ src\*.cpp -o main.exe -std=c++23 -g -mavx2 -DTESTS && gdb --ex=run --ex=bt --batch main.exe
	timeout /t 5
goto loop
