import os
import pandas as pd
import matplotlib.pyplot as plt

RESULTS_DIR = "results"
FIG_DIR = "figures"

os.makedirs(FIG_DIR, exist_ok=True)


def load_committee_files():

    dfs = []

    for f in os.listdir(RESULTS_DIR):

        if f.startswith("committee_") and f.endswith(".csv"):

            df = pd.read_csv(os.path.join(RESULTS_DIR, f))
            dfs.append(df)

    return pd.concat(dfs, ignore_index=True)


def load_request_files():

    dfs = []

    for f in os.listdir(RESULTS_DIR):

        if f.startswith("request_") and f.endswith(".csv"):

            df = pd.read_csv(os.path.join(RESULTS_DIR, f))
            dfs.append(df)

    return pd.concat(dfs, ignore_index=True)


# -----------------------------
# FIGURE 1: Latency vs Committee Size
# -----------------------------
def plot_latency_committee():

    df = load_committee_files()

    df = df.sort_values("committee_size")

    plt.figure()

    plt.plot(df["committee_size"], df["avg_latency_ms"], marker="o")

    plt.title("Latency vs Committee Size (O2R)")
    plt.xlabel("Committee Size")
    plt.ylabel("Average Latency (ms)")
    plt.grid(True)

    plt.savefig(os.path.join(FIG_DIR, "latency_committee.png"))
    plt.close()


# -----------------------------
# FIGURE 2: Approval Rate vs Committee Size
# -----------------------------
def plot_approval_committee():

    df = load_committee_files()

    df = df.sort_values("committee_size")

    plt.figure()

    plt.plot(df["committee_size"], df["approval_rate"], marker="o")

    plt.title("Approval Rate vs Committee Size")
    plt.xlabel("Committee Size")
    plt.ylabel("Approval Rate")
    plt.grid(True)

    plt.savefig(os.path.join(FIG_DIR, "approval_committee.png"))
    plt.close()


# -----------------------------
# FIGURE 3: Latency vs Request Load
# -----------------------------
def plot_latency_request():

    df = load_request_files()

    df = df.sort_values("total_requests")

    plt.figure()

    plt.plot(df["total_requests"], df["avg_latency_ms"], marker="o")

    plt.title("Latency vs Request Load")
    plt.xlabel("Number of Requests")
    plt.ylabel("Average Latency (ms)")
    plt.grid(True)

    plt.savefig(os.path.join(FIG_DIR, "latency_request.png"))
    plt.close()


if __name__ == "__main__":

    plot_latency_committee()
    plot_approval_committee()
    plot_latency_request()

    print("Plots saved to figures/")