# Train report for javascript / file:///tmp/top-repos-quality-repos-1r838adt/pslab-desktop.git HEAD 669f850676797920d012ad3047f6cc340b52c695

### Classification report

PPCR: 0.869

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.952| 0.995| 0.988| 0.973| 0.970| 33633| 33862| 0.993 |
| `␣` | 0.957| 0.906| 0.640| 0.931| 0.767| 7505| 10627| 0.706 |
| `'` | 1.000| 1.000| 0.987| 1.000| 0.994| 3481| 3526| 0.987 |
| `⏎␣⁻␣⁻` | 0.955| 0.710| 0.682| 0.815| 0.796| 1891| 1970| 0.960 |
| `"` | 1.000| 1.000| 1.000| 1.000| 1.000| 1472| 1472| 1.000 |
| `⏎` | 1.000| 0.734| 0.196| 0.847| 0.328| 940| 3513| 0.268 |
| `⏎␣⁺␣⁺` | 0.963| 0.557| 0.190| 0.706| 0.317| 657| 1928| 0.341 |
| `⏎⏎` | 0.944| 0.762| 0.604| 0.843| 0.737| 445| 561| 0.793 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 106| 0.047 |
| `micro avg` | 0.959| 0.959| 0.833| 0.959| 0.891| 50029| 57565| 0.869 |
| `macro avg` | 0.864| 0.740| 0.588| 0.791| 0.657| 50029| 57565| 0.869 |
| `weighted avg` | 0.959| 0.959| 0.833| 0.956| 0.864| 50029| 57565| 0.869 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎␣⁻␣⁻| ⏎␣⁺␣⁺| "| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|229 |33471 |141 |0 |0 |11 |10 |0 |0 |0 |
|3122 |654 |6797 |0 |0 |52 |2 |0 |0 |0 |
|2573 |155 |75 |690 |0 |0 |0 |0 |20 |0 |
|45 |0 |0 |0 |3481 |0 |0 |0 |0 |0 |
|79 |541 |5 |0 |0 |1343 |2 |0 |0 |0 |
|1271 |231 |60 |0 |0 |0 |366 |0 |0 |0 |
|0 |0 |0 |0 |0 |0 |0 |1472 |0 |0 |
|116 |88 |18 |0 |0 |0 |0 |0 |339 |0 |
|101 |1 |4 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/components/Appshell/Appshell.js | 199 |
| src/screen/WaveGenerator/components/DigitalController.js | 127 |
| src/screen/WaveGenerator/components/AnalogController.js | 96 |
| src/App.js | 95 |
| utils/preProcessor.js | 86 |
| src/screen/Oscilloscope/Oscilloscope.js | 80 |
| src/screen/AboutUs/AboutUs.js | 76 |
| src/screen/PowerSource/components/InstrumentCluster.js | 67 |
| src/screen/LoggedData/LoggedData.js | 61 |
| src/screen/LogicAnalyzer/LogicAnalyzer.js | 57 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1472}, "\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 3481}, "macro avg": {"f1-score": 0.7905123190556425, "precision": 0.8636045255065407, "recall": 0.7404411516861766, "support": 50029}, "micro avg": {"f1-score": 0.9586239980811129, "precision": 0.958623998081113, "recall": 0.958623998081113, "support": 50029}, "weighted avg": {"f1-score": 0.9564648788682383, "precision": 0.9588768998337648, "recall": 0.958623998081113, "support": 50029}, "\u2205": {"f1-score": 0.9733620263471661, "precision": 0.9524771634273356, "recall": 0.9951833021139952, "support": 33633}, "\u23ce": {"f1-score": 0.8466257668711656, "precision": 1.0, "recall": 0.7340425531914894, "support": 940}, "\u23ce\u23ce": {"f1-score": 0.8432835820895521, "precision": 0.9442896935933147, "recall": 0.7617977528089888, "support": 445}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.7058823529411765, "precision": 0.9631578947368421, "recall": 0.5570776255707762, "support": 657}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8146800121322414, "precision": 0.9551920341394026, "recall": 0.7102062400846113, "support": 1891}, "\u2423": {"f1-score": 0.9307771311194796, "precision": 0.9573239436619718, "recall": 0.9056628914057295, "support": 7505}},
  "cl_report_full": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1472}, "\u0027": {"f1-score": 0.9935778507207078, "precision": 1.0, "recall": 0.9872376630743052, "support": 3526}, "macro avg": {"f1-score": 0.6565144063890345, "precision": 0.8636045255065407, "recall": 0.5875043731885001, "support": 57565}, "micro avg": {"f1-score": 0.8914809375987508, "precision": 0.958623998081113, "recall": 0.8331277686093981, "support": 57565}, "weighted avg": {"f1-score": 0.8637370828059378, "precision": 0.9590151394716461, "recall": 0.8331277686093981, "support": 57565}, "\u2205": {"f1-score": 0.9701317334028955, "precision": 0.9524771634273356, "recall": 0.9884531333057704, "support": 33862}, "\u23ce": {"f1-score": 0.3283369022127052, "precision": 1.0, "recall": 0.1964133219470538, "support": 3513}, "\u23ce\u23ce": {"f1-score": 0.7369565217391304, "precision": 0.9442896935933147, "recall": 0.6042780748663101, "support": 561}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.31715771230502604, "precision": 0.9631578947368421, "recall": 0.18983402489626555, "support": 1928}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 106}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.7956161137440759, "precision": 0.9551920341394026, "recall": 0.6817258883248731, "support": 1970}, "\u2423": {"f1-score": 0.7668528233767697, "precision": 0.9573239436619718, "recall": 0.6395972522819234, "support": 10627}},
  "ppcr": 0.8690871189090593
}
```
</details>
