import numpy as np

N = 64

def float2fixed(x, m):
    return int(np.round(x * (2 ** m)))

def gen_roter_radix2():
    w = np.zeros(N, dtype=np.complex128)
    for i in range(N):
        w[i] = np.exp(2j * np.pi * i / N)
    return map(lambda x: float2fixed(x.real, 11), w), map(lambda x: float2fixed(x.imag, 11), w)

def signed_bin_format(n):
    if n < 0:
        return format((1 << 16) + n, '016b')
    else:
        return format(n, '016b')

def swap_bit_reverse(x):
    bit_rev_32 = [0, 16, 8, 24, 4, 20, 12, 28, 2, 18, 10, 26, 6, 22, 14, 30, 
                    1, 17, 9, 25, 5, 21, 13, 29, 3, 19, 11, 27, 7, 23, 15, 31]
    ret = []
    for i in bit_rev_32:
        ret.append(x[i])
    return ret

def main():
    r, i = gen_roter_radix2()
    r = list(r)
    i = list(i)
    r = swap_bit_reverse(r)
    i = swap_bit_reverse(i)
    c = 0
    for real, imag in zip(r, i):
        real_bin = signed_bin_format(real)
        imag_bin = signed_bin_format(imag)
        data = real_bin + imag_bin
        print('assign w[%d] = 32\'b%s;' % (c, data))
        c += 1
        if c == N // 2:
            break
if __name__ == '__main__':
    main()