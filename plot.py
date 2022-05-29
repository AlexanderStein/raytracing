#!/usr/bin/env python3

import argparse
import json
from turtle import color
from bokeh.layouts import row
from bokeh.plotting import figure, show
from bokeh.models import ColumnDataSource, Range1d


def main():
    parser = argparse.ArgumentParser(description='Plot hyperfine results')
    parser.add_argument('files', metavar='path', type=str, nargs='+',
                        help='Path to hpyerfine JSON output file')

    args = parser.parse_args()

    time_plot = figure(title="Raytracing time",
                       x_axis_label="# of threads", y_axis_label="total time [s]")
    speedup_plot = figure(
        title="Performance gain", x_axis_label="# of threads", y_axis_label="Speedup factor")

    max_threads = 0
    for file in args.files:
        with open(file) as file:
            data = json.load(file)
            threads = []
            time = []
            try:
                id = data['env']['id']
            except:
                id = "unknown"

            for run in data['results']:
                mean = float(run['mean'])
                num_threads = int(run['parameters']['threads'])
                time.append(mean)
                threads.append(num_threads)
                max_threads = max(max_threads, num_threads)

            data = {
                'time': time,
                'threads': threads,
            }
            time_data = ColumnDataSource(data=data)
            time_plot.line(x='threads', y='time',
                           source=time_data,
                           legend_label=f'{id}',
                           line_width=2
                           )
            time_plot.circle(x='threads', y='time',
                             source=time_data,
                             legend_label=f'{id}',
                             fill_color="white",
                             size=8)

            max_time_result = time_data.to_df().loc[0].get(['time', 'threads'])
            max_time = max_time_result[0]
            max_time_threads = max_time_result[1]
            gain = map(lambda item: max_time * max_time_threads / item, time)
            data = {
                'gain': list(gain),
                'threads': threads,
            }
            speedup_data = ColumnDataSource(data=data)
            speedup_plot.line(x='threads', y='gain',
                              source=speedup_data,
                              legend_label=f'{id}',
                              line_width=2)
            speedup_plot.circle(x='threads', y='gain',
                                source=speedup_data,
                                legend_label=f'{id}',
                                fill_color="white",
                                size=8)

    # Add 1 to have a small margin to the right border
    speedup_axis_max = max_threads + 1
    linear = {
        'threads': [0, speedup_axis_max],
        'gain': [0, speedup_axis_max]
    }
    speedup_plot.line(x='threads', y='gain',
                        source=ColumnDataSource(data=linear),
                        color='green',
                        alpha=0.3,
                        line_width=1)

    # Put legend to top left and the plot into top right corner
    speedup_plot.legend.location = "top_left"
    speedup_plot.x_range = Range1d(0, speedup_axis_max)
    speedup_plot.y_range = Range1d(0, speedup_axis_max)
    show(row(children=[time_plot, speedup_plot], sizing_mode="scale_width"))


if __name__ == '__main__':
    main()
