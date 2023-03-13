import matplotlib.pyplot as plt

nodes = [] # stores nodes in a list
edges = [] # stores edges in a list
switch = False # serves as marker to know when all nodes are read


with open("Graph.dat") as file:
    lines = file.readlines()

    # Iterate through each line of file
    for line in lines:
        line = line.strip() # (Removes leading spaces)

    # Between nodes and edges a blank line is added to mark the section
        if line == '': 
            switch = True
            continue

        if not switch:
            # there are still nodes to read
            no, x, y, id, marked = line.split()
            nodes.append( (int(no), float(x), float(y), str(id), bool(int(marked) == 1) ) )
        else:
            # all nodes have been read, so edges turn
            x1, y1, x2, y2, id, marked = line.split()
            edges.append( (float(x1), float(y1), float(x2), float(y2), str(id), bool(int(marked) == 1) ) )
            
# Convert data to matplotlib readable structures:
fig, ax = plt.subplots(figsize=(12, 12))

_, _, _, startnode, _ = nodes[0]

# Plot title
plt.title("Dijkstra Graph Plot (Start Node: " + startnode + ")", fontsize=30)

# Hide boths axis
ax.get_xaxis().set_visible(False)
ax.get_yaxis().set_visible(False)

# Convert nodes:
for node in nodes:
    no, x, y, id, marked = node # unpack from list
    color = 'red' if marked else 'blue' # set color for each node
    ax.scatter(x, y, s=100, color=color, label=id) # s = Markergröße (draw single points)
    ax.annotate(id, (x, y), textcoords='offset points', xytext=(0, 10), ha='center', fontsize=16 ) # add it to plot

# Convert edges:
for edge in edges:
    x1, y1, x2, y2, weight, marked = edge # unpack
    color = 'red' if marked else 'blue' # set color for each edge
    mid_x = (x1 + x2) / 2 # calculate position of label
    mid_y = (y1 + y2) / 2
    ax.plot([x1, x2], [y1, y2], linewidth=4, color=color, label=weight) # linewidth = Linienbreite (draw lines)
    ax.annotate(str(weight), (mid_x, mid_y), textcoords='offset points', xytext=(0, 10), ha='center', fontsize=14) # add it to plot

# Add legend:
legend_elements = [plt.Line2D([0], [0], marker='o', color='w', label='Unmarked Node', markerfacecolor='blue', markersize=15),
                   plt.Line2D([0], [0], marker='o', color='w', label='Marked Node', markerfacecolor='red', markersize=15),
                   plt.Line2D([0], [0], color='blue', label='Unmarked Edge', linewidth=4),
                   plt.Line2D([0], [0], color='red', label='Marked Edge', linewidth=4)]
ax.legend(handles=legend_elements, loc='best', fontsize=14)

# Save plot:
plt.savefig("graph.png")

# Show plot:
plt.show()