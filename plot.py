import os  # check file
import sys  # getting arguments / exit script
import matplotlib.pyplot as plt  # for plotting

# Variable declaration
nodes = []  # stores nodes in a list
edges = []  # stores edges in a list
switch = False  # serves as marker to know when all nodes are read
file_in = "" # data file with coordinates (read in as parameter)
file_out = "" # png file in which plot is saved (read in as parameter)
name = sys.argv[0]

# If only one argument has passed exit script000
if len(sys.argv) < 2:
    print(f"{name}: No parameters specified!")
    sys.exit(1)

# Iterate through all parameter
for arg in sys.argv[1:]:
    if arg.startswith("-input="):
        file_in = arg[len("-input="):]  # Slice
    if arg.startswith("-output="):
        file_out = arg[len("-output="):]  # Slice

# Debug:
#print(f"input: {file_in}, output: {file_out}")

# Abort script if critical information are missing/faulty:
if file_in is None or file_out is None or not os.path.isfile(file_in):
    print(f"\033[1m{name}:\033[0m Invalid parameter values and/or source file doesn't exist!")
    sys.exit(1)

# Start of actual work:

# Open graph data file:
with open(file_in) as file:
    lines = file.readlines()

    # Iterate through each line of file
    for line in lines:
        line = line.strip()  # (Removes leading spaces)

        # Between nodes and edges a blank line is added to mark the section (therefore it is absolutley necessary that source file is not changed manually!)
        if line == '':
            switch = True
            continue

        if not switch:
            # there are still nodes to read
            no, x, y, id, marked = line.split()
            nodes.append((int(no), float(x), float(y), str(id), bool(int(marked) == 1)))
        else:
            # all nodes have been read, so turn to edges
            x1, y1, x2, y2, id, marked = line.split()
            edges.append((float(x1), float(y1), float(x2), float(y2), str(id), bool(int(marked) == 1)))

# Convert data to matplotlib readable structures:
fig, ax = plt.subplots(figsize=(12, 12))

_, _, _, startnode, _ = nodes[0]  # unpack first entry (is by definition start node)

# Plot title
plt.title("Dijkstra Graph Plot (Start Node: " + startnode + ")", fontsize=30)

# Hide boths axis
ax.get_xaxis().set_visible(False)
ax.get_yaxis().set_visible(False)

# Convert and add edges to plot:
for edge in edges:
    x1, y1, x2, y2, weight, marked = edge  # unpack
    color = 'red' if marked else 'blue'  # set color for each edge
    mid_x = (x1 + x2) / 2  # calculate position of label
    mid_y = (y1 + y2) / 2
    ax.plot([x1, x2], [y1, y2], linewidth=4, color=color, label=weight,
            zorder=1)  # linewidth = Linienbreite (draw lines)
    ax.annotate(str(weight), (mid_x, mid_y), textcoords='offset points', xytext=(0, 10), ha='center',
                fontsize=14)  # add it to plot

# Convert and add nodes to plot:
for node in nodes:
    no, x, y, id, marked = node  # unpack from list
    color = 'red' if marked else 'blue'  # set color for each node
    ax.scatter(x, y, s=120, color=color, label=id,
               zorder=2)  # s = Markergröße (draw single points); zorder=2 objects are drawn over zorder=1
    ax.annotate(id, (x, y), textcoords='offset points', xytext=(0, 10), ha='center', fontsize=16)  # add it to plot

# Add legend:
legend_elements = [
    plt.Line2D([0], [0], marker='o', color='w', label='Unmarked Node', markerfacecolor='blue', markersize=15),
    plt.Line2D([0], [0], marker='o', color='w', label='Marked Node', markerfacecolor='red', markersize=15),
    plt.Line2D([0], [0], color='blue', label='Unmarked Edge', linewidth=4),
    plt.Line2D([0], [0], color='red', label='Marked Edge', linewidth=4)]
ax.legend(handles=legend_elements, loc='best', fontsize=14)

# Save plot:
plt.savefig(file_out)

print(f"\033[1m{name}:\033[0m Plot was saved at \033[1m{file_out}\033[0m")

# Show plot:
plt.show()