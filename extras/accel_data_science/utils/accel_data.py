import pandas as pd

def parse_accel_logs(data):
    accel_start = "accel: "
    lines = data.split("\n")
    result = []
    for line in lines:
        if line.startswith(accel_start):
            axis = [int(x) for x in line[len(accel_start):].split("x")]
            result.append(axis)
    return pd.DataFrame(result, index=range(0, len(result)), columns=list("XYZ"))