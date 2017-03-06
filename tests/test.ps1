# Package the native_test binary
..\target\debug\nuget.exe --cargo-dir .\native\ --nupkg-dir .\feed\

# Remove any cached native_test packages
Remove-Item $HOME\.nuget\packages\native_test -recurse

# Restore dotnet pkgs and run
cd dotnet
dotnet restore --configfile .\Nuget.config
dotnet run
cd ..\