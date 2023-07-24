import numpy as np

N = 64

def float2fixed(x, m):
    return int(np.round(x * (2 ** m)))

def gen_roter_radix2():
    w = np.zeros(N, dtype=np.float64)
    for i in range(N):
        w[i] = np.exp(4j * np.pi * i / N).imag / 8
    return map(lambda x: float2fixed(x, 11), w)

def signed_bin_format(n):
    if n < 0:
        return format((1 << 16) + n, '016b')
    else:
        return format(n, '016b')

def main():
    input = gen_roter_radix2()
    for i, data in enumerate(input):
        print(signed_bin_format(data))
if __name__ == '__main__':
    main()