# 🏎️ F1 Championship Recalculator

A Rust-based application to **recalculate Formula 1 championship standings** with customizable point systems, including sprint races, fastest laps, and poles.

---

## **Features**

- Recalculate championship standings based on:
  - Grand Prix results
  - Sprint results
  - Fastest laps (Including Sprints)
  - Pole positions (Including Sprints)
- Supports **custom point systems** via configuration .toml file.
- **Caches race results** to speed up repeated calculations.
- Display standings in a **clear, tabular format**, including:
  - Driver name  
  - Total points  
  - Wins / Sprint wins  
  - Fastest laps / Sprint fastest laps  
  - Poles / Sprint poles  
- Fetches data from the **[Jolpica F1 API](https://api.jolpi.ca/ergast/)**.
---

## **Installation**

1. Go to the **[GitHub Releases](https://github.com/ZyreXINF/F1_championship_recalculator/releases)** page.
2. Download the latest **`f1_championship_recalculator.exe`** file.
3. Place it anywhere on your system.
4. Run it by double-clicking the **`.exe`** or via terminal:

```bash
./f1_championship_recalculator.exe
```
---

## **Configuration**

The app uses a TOML configuration file to define point systems. Example configuration:

```toml
# config.toml

fastest_lap_point = 0
sprint_fastest_lap_point = 0
pole_point = 0
sprint_pole_point = 0

[gp_point_allocation]
1 = 25
2 = 18
3 = 15
4 = 12
5 = 10
6 = 8
7 = 6
8 = 4
9 = 2
10 = 1

[sprint_point_allocation]
1 = 8
2 = 7
3 = 6
4 = 5
5 = 4
6 = 3
7 = 2
8 = 1
```

### Parameters explained

***fastest_lap_point***: Points for the fastest lap in a GP.

***sprint_fastest_lap_point***: Points for the fastest lap in a sprint race.

***pole_point***: Points for pole position in a GP.

***sprint_pole_point***: Points for pole position in a sprint race.

***gp_point_allocation***: Points for finishing positions in a Grand Prix.

***sprint_point_allocation***: Points for finishing positions in sprint races.

**⚠️ Make sure your TOML keys are numbers corresponding to the finishing positions.**

---

## **Usage**

Run the **executable**.

Select the year of the championship.

The app will:

- Fetch results from the API (cached for future runs)

- Recalculate standings using your point system

- Display standings in a table:

```
Pos  Driver                   Pts  W  W(S) FL FL(S) Poles Poles(S)
--------------------------------------------------------------------
1    Max Verstappen           395  14   3   5    1    7      2
2    Lewis Hamilton           320  8    1   2    0    4      1
...
```
---
### **License**

This project is released under the **[MIT](https://choosealicense.com/licenses/mit/)**.
