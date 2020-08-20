import json
import matplotlib.pyplot as plt
import numpy as np
import sys

def main(argv):
    if len(sys.argv) != 4:
        return
    
    src = json.loads(argv[1])
    filepath = argv[2]
    username = argv[3]
    gen_graph(src, filepath, username)

def gen_graph(src, filepath, username):
    # src = "[[\"10/08/2020\", 3],[\"12/08/2020\", 14],[\"13/08/2020\", 5],[\"14/08/2020\", 4],[\"15/08/2020\", 20],[\"16/08/2020\", 4]]"
    # src = json.loads(src)
    labels = []
    data = []
    for hist in src:
        labels.append(hist[0])
        data.append(hist[1])

    fig, ax = plt.subplots()
    ax.bar(labels, data, align='center', width=0.5)
    ax.set_xticklabels(labels)
    ax.set_ylabel('Malds')
    ax.set_title(username + '\'s malds over time')
    plt.savefig(filepath, format='png')


if __name__ == "__main__":
    main(sys.argv)