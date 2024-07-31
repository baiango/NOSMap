:loop
	cls
	g++ src\*.cpp -o main.exe -O2 -mavx2 && main.exe
	timeout /t 5
goto loop
