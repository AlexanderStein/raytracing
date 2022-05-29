#!/usr/bin/env python3

import argparse
import json
from bokeh.plotting import figure, show

def main():
    #TODO: plot multiple result files using nargs='+',
    parser = argparse.ArgumentParser(description='Plot hyperfine results')
    parser.add_argument('file', metavar='path', type=str,
                        help='Path to hpyerfine JSON output file')

    args = parser.parse_args()

    p = figure(title="Raytracing time", x_axis_label="# of threads", y_axis_label="time [s]")

    with open(args.file) as file:
        data = json.load(file)
        threads = []
        time = []
        try:
            id = data['env']['id']
        except:
            id = "unknown"

        for run in data['results']:
            time.append(float(run['mean']))
            threads.append(int(run['parameters']['threads']))

        p.line(threads, time, legend_label=f'{id}', line_width=2)

    show(p)

if __name__ == '__main__':
    main()
