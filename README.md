# CACHEDIR.TAG mark - get_cachedir_list
A simple program that lists all the directories that are cache tagged. 

## Description
This is a little program that lists on a file one line for each directory entry the CACHEDIR.TAG mark on the files that are of the type cache. <br>
For information about cache directory tags see <br>
[https://bford.info/cachedir/](https://bford.info/cachedir/) <br>
The search is made for the sub-directories of the given directory. The target for the time being is Linux. <br>

```
Usage example:
      get_cachedir_list  <search dir> <dir to place the result file>

   ex:
      It writes by default on dir /dev/shm  join  /exclusion_dirs.txt RAM Disk on Linux.
      get_cachedir_list  /home/<user>

   ex:
      You can also specify the directory to write to.
      get_cachedir_list  /home/<user> /dev/shm/bla 
```

## License
MIT Open Source License.

## Have fun
Best regards, <br>
Joao Nuno Carvalho

