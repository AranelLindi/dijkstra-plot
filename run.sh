#!/bin/bash

# This script was made to start dijkstra-plot application.
# To do this it requests various input parameters such as GraphML-input-file,
# data output file and dijkstra algorithm-specific information such as start/destination
# node. It then runs a python script that draws the computed graph and outputs
# it to a separate window. A png file of the output is also be created.
# Stefan Lindörfer - 2023



# 1. Read in necessary information from user:
read -p $'\e[1mInsert GraphML-File:\e[0m ' input_path
read -p $'\e[1mInsert Data Output File:\e[0m ' output_path
read -p $'\e[1mInsert Start Node:\e[0m ' start
read -p $'\e[1mInsert Destination Node:\e[0m ' dest

# 1.1 Check inputs for validity:
if [ -z "$input_path" ] || ! [ -e "$(pwd)/$input_path" ]; then # Checks if input_path is empty or no file exists under the path
	echo -e "\033[1m$BASH_SOURCE:\033[0m \033[31mInvalid GraphML-file parameter entered!\033[0m"
	exit 1
fi
if [ -z "$output_path" ]; then # Checks only if output path is empty and assigns them a default value instead
  output_path="Graph.dat" # Important: no spaces between expression are allowed! (e.g. input_path = "Graph.xml") otherwise each construct is interpreted as an command!
fi

# At this point: All information are valid!

# 2. Announce start of rust application to perform dijkstra algorithm...
echo -e "\033[1m$BASH_SOURCE:\033[0m Dijkstra Plot is now going to be executed..."

# 2.1 Execute dijkstra-plot with given parameters
./target/debug/dijkstra-plot -input="$input_path" -output="Graph.dat" -start="$start" -dest="$dest" # & means that application script continues while app is running

# 2.2 Check if application returned with error code
if [ $? -eq 1 ]; then # $? is a shell variable that contains exit code of last executed command (here: application)
  echo -e "\033[1m$BASH_SOURCE:\033[0m \033[31mError in rust application!\033[0m"
  exit 1 # End script with error code 1
else
  echo -e "\033[1m$BASH_SOURCE:\033[0m Dijkstra Plot executed successfully!"
fi

# 2.3 Check if Graph.dat was created in root directory (Graph.dat is a constant value, output file from rust application is always named as "Graph.dat"!)
if [ -f "Graph.dat" ]; then
	echo -e "\033[1m$BASH_SOURCE:\033[0m Graph.dat was created!"
else
	echo -e "\033[1m$BASH_SOURCE:\033[0m \033[31mGraph.dat was not created!\033[0m"
fi

# 3. Check if matlibplot library is installed and if yes: Run python script to create plot
if [ $(pip show matplotlib >/dev/null 2>&1; echo $?) -eq 0 ]; then
    # Run python script (that writes specific information into output stream)
    python3 plot.py -input=$input_path -output="$output_path"
  else
    echo -e "\033[1m$BASH_SOURCE:\033[0m \033[31mPython script couldn't be executed because matlibplot library isn't installed on the system!\033[0m"
fi

# 4. Finished!
echo -e "\033[1m$BASH_SOURCE: Finished!\033[0m"