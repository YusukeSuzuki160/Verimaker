import numpy as np
import matplotlib.pyplot as plt
import pandas as pd

filename = 'input'

def read_csv_as_bin(filename: str) -> pd.DataFrame:
    df = pd.read_csv(filename, header=None)
    df = df.applymap(lambda x: str(x))
    return df

def fixed2float(x: int, m: int) -> float:
    if len(x) == 16 and x[0] == '1':
        x = int(x, 2) - (1 << 16)
    else:
        x = int(x, 2)
    return x / (2 ** m)
    
def main():
    df = read_csv_as_bin(filename + '.txt')
    df = df.applymap(lambda x: fixed2float(x, 11))
    df.plot()
    plt.xlabel('Time')
    plt.ylabel('Amplitude')
    plt.savefig(filename + '.png')

if __name__ == '__main__':
    main()