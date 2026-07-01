"""Render charts from the comparison CSV files.

Reads every CSV the comparison harnesses write to comparisons/results/ (see
`make bench-graphina`, `make bench-pygraphina`, and their dataset variants) and
saves one PNG per benchmark run under comparisons/results/plots/. Each chart is a
horizontal grouped bar chart of the median wall time per algorithm and library,
on a logarithmic time axis, with error bars showing the 95% bootstrap confidence
interval of the median.

Usage:
    uv run --with matplotlib python comparisons/visualize.py [results_dir]
"""

from __future__ import annotations

import csv
import os
import sys

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

# Fixed library order and colors so charts from different runs are comparable.
LIBRARIES = ["graphina", "pygraphina", "rustworkx-core", "rustworkx", "networkx"]
COLORS = {
    "graphina": "#1f77b4",
    "pygraphina": "#1f77b4",
    "rustworkx-core": "#ff7f0e",
    "rustworkx": "#ff7f0e",
    "networkx": "#2ca02c",
}


def load_runs(csv_path: str) -> dict[tuple[str, str, str], list[dict[str, str]]]:
    """Group the CSV rows by benchmark run. A file usually holds one run; a sweep
    holds one run per size, distinguished by the (dataset, nodes, edges) key.
    """
    runs: dict[tuple[str, str, str], list[dict[str, str]]] = {}
    with open(csv_path, newline="", encoding="utf-8") as f:
        for row in csv.DictReader(f):
            key = (row["dataset"], row["nodes"], row["edges"])
            runs.setdefault(key, []).append(row)
    return runs


def plot_run(
    rows: list[dict[str, str]], title: str, out_path: str
) -> None:
    """Draw one grouped bar chart for a single benchmark run."""
    # Algorithms in file order, keeping only those with at least one timed library.
    algorithms: list[str] = []
    timings: dict[tuple[str, str], tuple[float, float, float]] = {}
    untimed: list[str] = []
    for row in rows:
        name = row["algorithm"]
        if name not in algorithms and name not in untimed:
            untimed.append(name)
        if row["median_s"]:
            timings[(name, row["library"])] = (
                float(row["median_s"]),
                float(row["ci_lo_s"]),
                float(row["ci_hi_s"]),
            )
            if name not in algorithms:
                algorithms.append(name)
                untimed.remove(name)

    if not algorithms:
        print(f"  no timed rows for {title}; skipping")
        return

    libraries = [
        lib for lib in LIBRARIES if any((a, lib) in timings for a in algorithms)
    ]
    bar_height = 0.8 / len(libraries)
    fig_height = max(3.0, 0.45 * len(algorithms) + 1.5)
    fig, ax = plt.subplots(figsize=(10, fig_height))

    for li, lib in enumerate(libraries):
        ys, medians, err_lo, err_hi = [], [], [], []
        for ai, algo in enumerate(algorithms):
            stat = timings.get((algo, lib))
            if stat is None:
                continue
            median, ci_lo, ci_hi = stat
            ys.append(ai + (li - (len(libraries) - 1) / 2) * bar_height)
            medians.append(median)
            err_lo.append(max(median - ci_lo, 0.0))
            err_hi.append(max(ci_hi - median, 0.0))
        ax.barh(
            ys,
            medians,
            height=bar_height,
            xerr=(err_lo, err_hi),
            color=COLORS.get(lib, "#7f7f7f"),
            label=lib,
            error_kw={"linewidth": 0.8, "capsize": 2},
        )

    ax.set_yticks(range(len(algorithms)))
    ax.set_yticklabels(algorithms)
    ax.set_ylim(len(algorithms) - 0.5, -0.5)
    ax.set_xscale("log")
    ax.set_xlabel("median wall time (s, log scale; lower is better)")
    if untimed:
        title = f"{title}\nnot timed (skipped, mismatch, or error): {', '.join(untimed)}"
    ax.set_title(title, fontsize=10)
    ax.legend(loc="lower right", fontsize=8)
    ax.grid(axis="x", which="both", alpha=0.3)
    fig.tight_layout()
    fig.savefig(out_path, dpi=150)
    plt.close(fig)
    print(f"  wrote {out_path}")


def main() -> None:
    results_dir = sys.argv[1] if len(sys.argv) > 1 else "comparisons/results"
    csv_files = sorted(
        os.path.join(results_dir, f)
        for f in os.listdir(results_dir)
        if f.endswith(".csv")
    ) if os.path.isdir(results_dir) else []
    if not csv_files:
        sys.exit(
            f"no CSV files in {results_dir}; run `make bench-graphina` or "
            "`make bench-pygraphina` first"
        )

    plots_dir = os.path.join(results_dir, "plots")
    os.makedirs(plots_dir, exist_ok=True)

    for csv_path in csv_files:
        print(f"{csv_path}:")
        stem = os.path.splitext(os.path.basename(csv_path))[0]
        runs = load_runs(csv_path)
        for (dataset, nodes, edges), rows in runs.items():
            suffix = f"-{nodes}n" if len(runs) > 1 else ""
            title = f"{stem}: {dataset} ({nodes} nodes, {edges} edges)"
            out_path = os.path.join(plots_dir, f"{stem}{suffix}.png")
            plot_run(rows, title, out_path)


if __name__ == "__main__":
    main()
