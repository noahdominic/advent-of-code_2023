import numpy as np
from sklearn.preprocessing import PolynomialFeatures
from sklearn.linear_model import LinearRegression
from sklearn.metrics import mean_squared_error

def get_prev(sequence):
    diffs = sequence 
    prev = diffs[0]
    is_subtr = True
    while not all(x==0 for x in diffs):
        diffs = [next - curr for curr, next in zip(diffs[:-1], diffs[1:])]
        prev = prev - diffs[0] if is_subtr else prev + diffs[0]
        is_subtr = not is_subtr
   
    return prev

def get_next(sequence):
    diffs = sequence 
    degree = 1
    next = diffs[-1]
    while not all(x==0 for x in diffs):
        diffs = [next - curr for curr, next in zip(diffs[:-1], diffs[1:])]
        next += diffs[-1]
        degree += 1
   
    return next

def get_deg(sequence):
    diffs = sequence 
    degree = 1
    while not any(x==0 for x in diffs):
        diffs = [next - curr for curr, next in zip(diffs[:-1], diffs[1:])]
        degree += 1
   
    return degree

def predict_next_number(sequence):
    # Creating features and target
    X = np.array(range(1, len(sequence) + 1)).reshape(-1, 1)
    y = np.array(sequence)

    # Creating polynomial features
    poly = PolynomialFeatures(degree=get_deg(sequence))
    X_poly = poly.fit_transform(X)

    # Fitting the polynomial regression model
    model = LinearRegression()
    model.fit(X_poly, y)

    # Predicting the next number
    next_number = model.predict(poly.transform([[len(sequence) + 1]]))
    predicted_number = int(round(next_number[0]))

    # Calculating RMSE
    y_pred = model.predict(X_poly)
    rmse = np.sqrt(mean_squared_error(y, y_pred))

    return predicted_number, rmse

with open("input.txt") as f:
    lines = f.readlines()

lines = [[int(num) for num in ln.split()] for ln in lines]

total1 = 0
total2 = 0

for sequence in lines:
    actual_next_number = get_next(sequence)
    actual_prev_number = get_prev(sequence)
    total1 += actual_next_number
    total2 += actual_prev_number
    # print("Next: {0}\tPrev:{1}".format(actual_next_number, actual_prev_number))
    print("Next: {0}\tPrev: {1}".format(actual_next_number, actual_prev_number))

print(total1)
print(total2)
