import io
import os
from time import monotonic
from collections import namedtuple

import tabulate

Result = namedtuple('Result', 'path iterations benchmark_results')
BenchmarkResult = namedtuple('BenchmarkResult', 'name duration')


ROOT = os.path.abspath(os.path.dirname(__file__))

PATHS = [
    os.path.join(ROOT, 'imgsize', 'testdata', 'lenna.jpg'),
    os.path.join(ROOT, 'imgsize', 'testdata', 'lenna.png'),
    os.path.join(ROOT, 'imgsize', 'testdata', 'lenna.gif'),
    os.path.join(ROOT, 'imgsize', 'testdata', 'lenna.bmp'),
]


class ImgsizeBenchmark(object):
    name = 'imgsize'
    def __init__(self):
        import imgsize
        self.get_size = imgsize.get_size
        
    def run(self, fpath):
        with io.open(fpath, 'rb') as fobj:
            self.get_size(fobj)


class PillowBenchmark(object):
    name = 'Pillow'
    def __init__(self):
        from PIL import Image
        self.image = Image

    def run(self, fpath):
        im = self.image.open(fpath)
        im.size


def benchmark(benchmarks, paths, iters):
    results = []
    for path in paths:
        benchmark_results = []
        for bench in benchmarks:
            total = 0.0
            for _ in range(iters):
                start = monotonic()
                bench.run(path)
                end = monotonic()
                total += (end - start)
            benchmark_results.append(BenchmarkResult(bench.name, total))
        results.append(Result(path, iters, benchmark_results))
    return results


def main():
    imgsize_bench = ImgsizeBenchmark()
    pillow_bench = PillowBenchmark()
    results = benchmark([imgsize_bench, pillow_bench], PATHS, 10000)
    table = []
    for result in results:
        imgsize, pillow = result.benchmark_results
        table.append([
            result.path.rsplit('.')[-1],
            imgsize.duration,
            pillow.duration,
            '%.2fx' % (pillow.duration / imgsize.duration),
        ])
    headers = ['format', 'imgsize', 'pillow', 'speedup']
    print(tabulate.tabulate(table, headers=headers, floatfmt='.4f'))


if __name__ == '__main__':
    main()

