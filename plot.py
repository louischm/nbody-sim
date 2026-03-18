import pandas as pd
import matplotlib.pyplot as plt

df = pd.read_csv("output.csv")

for body in df["body"].unique():
    subset = df[df["body"] == body]
    plt.plot(subset["x"], subset["y"], label=body)

plt.legend()
plt.xlabel("x")
plt.ylabel("y")
plt.title("Orbit simulation")
plt.axis("equal")
plt.show()