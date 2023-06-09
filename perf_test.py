import subprocess
import time
import matplotlib.pyplot as plt
import numpy as np

release_bin = ('./target/debug/json_faster', '--json-size=')
release_bin = ('./target/release/json_faster')


def build_args(debug=True, json_size=10, method='naive'):
    if debug:
        return ('./target/debug/json_faster', f'--json-size={json_size}')

    return ('./target/release/json_faster', f'--json-size={json_size}', f'--method={method}')


def parse_result(res):
    if isinstance(res, bytes):
        res = res.decode()
    if not isinstance(res, str):
        raise ValueError('result to be parsed must be of type str')
    hdr, row, _ = res.split('\n')
    size, time = row.split(',')
    return int(size), float(time)


def test_release():
    json_sizes = [10, 100, 1000, 5000]
    result = []
    for size in json_sizes:
        c_times = []
        w_times = []
        for _ in range(10):
            args = build_args(debug=False, json_size=size, method='naive')
            p = subprocess.run(args, capture_output=True)
            _, whole_time = parse_result(p.stderr)
            time.sleep(0.5)
            args = build_args(debug=False, json_size=size, method='custom')
            p = subprocess.run(args, capture_output=True)
            _, custom_time = parse_result(p.stderr)
            c_times.append(custom_time)
            w_times.append(whole_time)

        custom_avg = sum(c_times) / len(c_times)
        whole_avg = sum(w_times) / len(w_times)

        result.append((size, custom_avg, whole_avg))

        print(
            f'size = {size}, iterated {10} times for 1000-iterated avg measurement')
        print(f'custom parser = {custom_avg}')
        print(f'whole parser =  {whole_avg}')
        print(f'whole / custom = {whole_avg / custom_avg}')

    return result


def plot_test_result(test_result):
    sizes, avg1, avg2 = zip(*test_result)

    # Set up plot
    fig, ax = plt.subplots()
    bar_width = 0.35
    index = np.arange(len(sizes))

    # Create the bars for avg1 and avg2
    bar_avg1 = ax.bar(index, avg1, bar_width, label='Custom Parsing Average')
    bar_avg2 = ax.bar(index + bar_width, avg2, bar_width,
                      label='Whole Parsing Average')

    # Set up the x-axis labels, title, and legend
    ax.set_xticks(index + bar_width / 2)
    ax.set_xticklabels(sizes)
    ax.set_title('Comparison of average parsing time for different sizes')
    ax.legend()

    # Show the plot
    plt.show()

    # Clear the figure for later test plots
    plt.clf()


if __name__ == "__main__":
    # print("Test DEBUG build...")
    # test_debug_result = test_debug()
    # plot_test_result(test_debug_result)

    # plt.clf()

    print("Test RELEASE build...")
    test_release_result = test_release()
    plot_test_result(test_release_result)
