import sys
from parse import parse

lines = sys.stdin.readlines()
print("Finished reading")
started = False
times = {}
errors = {}
axes = []

for x in lines:
    if x == "running 81 tests\n":
        print("Found Start")
        started = True
        continue
    if started:
        # Example "test perft_01_board          ... bench:   6,760,185 ns/iter (+/- 331,568)"
        result = parse("testperft_{}_{}...bench:{}ns/iter(+/-{})", x.replace(',', '').replace(" ", ""))
        try:
            test_number = result[0]
            program = result[1]
            time = int(result[2]) / 1000000.0
            error = int(result[3]) / 1000000.0

            # print("%s on test %s ran in %i with error %i." % (program, test_number, time, error))

            if program not in times:
                times[program] = []
                errors[program] = []

            if test_number not in axes:
                axes.append(test_number)
            times[program].append(time)
            errors[program].append(error)
        except:
            break

import numpy as np
import matplotlib.pyplot as plt

fig, ax = plt.subplots()

N = len(times["chess"])
ind = np.arange(N)
height = 0.25

shakmaty_rects = ax.barh(ind, times["shakmaty"], height, color='r', xerr=errors["shakmaty"])
#chess_incremental_rects = ax.barh(ind + height, times["chess_incremental"], height, color='b', xerr=errors["chess_incremental"])
chess_rects = ax.barh(ind + height, times["chess"], height, color='g', xerr=errors["chess"])

ax.set_ylabel('Test')
ax.set_xlabel('Time (in s)')
ax.set_title('Times by move generator')
ax.set_yticks(ind + height)
ax.set_yticklabels(axes)
ax.legend((shakmaty_rects, chess_rects), ('shakmaty', 'chess'))

def autolabel(rects):
    for rect in rects:
        ax.text(rect.get_width(), rect.get_y() + rect.get_height() / 2., '%.4f' % rect.get_width(), ha='left', va='bottom', size = 6)


def adjustFigAspect(fig,aspect=1):
    '''
    Adjust the subplot parameters so that the figure has the correct
    aspect ratio.
    '''
    xsize,ysize = fig.get_size_inches()
    minsize = min(xsize,ysize)
    xlim = .4*minsize/xsize
    ylim = .4*minsize/ysize
    if aspect < 1:
        xlim *= aspect
    else:
        ylim /= aspect
    fig.subplots_adjust(left=.5-xlim,
                        right=.5+xlim,
                        bottom=.5-ylim,
                        top=.5+ylim)

autolabel(shakmaty_rects)
autolabel(chess_rects)
#autolabel(chess_incremental_rects)

plt.rcParams["figure.figsize"] = [20, 1000]
adjustFigAspect(fig, aspect=0.25)
plt.show()

