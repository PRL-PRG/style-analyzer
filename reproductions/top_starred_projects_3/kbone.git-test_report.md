# Test report for javascript / file:///tmp/top-repos-quality-repos-nlskh9c9/kbone.git HEAD 03e72e770dff9b83be6ad9c4813dc08e80dbd797

### Classification report

PPCR: 0.979

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.989| 0.974| 0.966| 0.982| 0.978| 21148| 21316| 0.992 |
| `␣` | 0.903| 0.978| 0.975| 0.939| 0.938| 12009| 12053| 0.996 |
| `'` | 1.000| 0.980| 0.980| 0.990| 0.990| 4624| 4624| 1.000 |
| `⏎` | 0.839| 0.814| 0.747| 0.827| 0.791| 2633| 2869| 0.918 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.875| 0.934| 0.890| 0.904| 0.883| 1128| 1184| 0.953 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.892| 0.930| 0.813| 0.911| 0.851| 1031| 1180| 0.874 |
| `⏎⏎` | 0.917| 0.473| 0.426| 0.624| 0.582| 862| 958| 0.900 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 100| 165| 0.606 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 90| 163| 0.552 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 44| 0.364 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 44| 0.136 |
| `weighted avg` | 0.946| 0.950| 0.929| 0.946| 0.933| 43647| 44600| 0.979 |
| `micro avg` | 0.950| 0.950| 0.929| 0.950| 0.939| 43647| 44600| 0.979 |
| `macro avg` | 0.583| 0.553| 0.527| 0.561| 0.546| 43647| 44600| 0.979 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⇥⁺| ⏎⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|168 |20601 |412 |0 |19 |70 |46 |0 |0 |0 |0 |0 |
|44 |112 |11750 |0 |137 |7 |3 |0 |0 |0 |0 |0 |
|0 |23 |68 |4533 |0 |0 |0 |0 |0 |0 |0 |0 |
|236 |19 |430 |0 |2144 |2 |1 |37 |0 |0 |0 |0 |
|56 |40 |26 |1 |7 |1054 |0 |0 |0 |0 |0 |0 |
|149 |10 |62 |0 |0 |0 |959 |0 |0 |0 |0 |0 |
|96 |3 |212 |0 |239 |0 |0 |408 |0 |0 |0 |0 |
|65 |9 |23 |1 |0 |67 |0 |0 |0 |0 |0 |0 |
|73 |7 |17 |0 |0 |0 |66 |0 |0 |0 |0 |0 |
|28 |0 |4 |0 |8 |4 |0 |0 |0 |0 |0 |0 |
|38 |0 |6 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 0.9898460530625615, "precision": 0.9995589856670342, "recall": 0.9803200692041523, "support": 4624}, "macro avg": {"f1-score": 0.561497782291681, "precision": 0.5832573954816954, "recall": 0.553186162439653, "support": 43647}, "micro avg": {"f1-score": 0.9496414415652852, "precision": 0.9496414415652852, "recall": 0.9496414415652852, "support": 43647}, "weighted avg": {"f1-score": 0.9460078426860619, "precision": 0.9461658050718986, "recall": 0.9496414415652852, "support": 43647}, "\u2205": {"f1-score": 0.9816544362908607, "precision": 0.9892912024587015, "recall": 0.9741346699451485, "support": 21148}, "\u23ce": {"f1-score": 0.8266820898399846, "precision": 0.8394675019577134, "recall": 0.8142802886441322, "support": 2633}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u23ce\u23ce": {"f1-score": 0.6243305279265493, "precision": 0.9168539325842696, "recall": 0.4733178654292343, "support": 862}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 100}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.9039451114922814, "precision": 0.8754152823920266, "recall": 0.9343971631205674, "support": 1128}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 90}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.9107312440645775, "precision": 0.892093023255814, "recall": 0.930164888457808, "support": 1031}, "\u2423": {"f1-score": 0.9392861425316759, "precision": 0.90315142198309, "recall": 0.9784328420351404, "support": 12009}},
  "cl_report_full": {"\u0027": {"f1-score": 0.9898460530625615, "precision": 0.9995589856670342, "recall": 0.9803200692041523, "support": 4624}, "macro avg": {"f1-score": 0.5464402317002265, "precision": 0.5832573954816954, "recall": 0.5270671603328518, "support": 44600}, "micro avg": {"f1-score": 0.9393860414518342, "precision": 0.9496414415652852, "recall": 0.9293497757847534, "support": 44600}, "weighted avg": {"f1-score": 0.9326111898711689, "precision": 0.9410608946821225, "recall": 0.9293497757847534, "support": 44600}, "\u2205": {"f1-score": 0.9777408637873755, "precision": 0.9892912024587015, "recall": 0.9664571214111466, "support": 21316}, "\u23ce": {"f1-score": 0.7907062511524985, "precision": 0.8394675019577134, "recall": 0.747298710352039, "support": 2869}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 44}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 44}, "\u23ce\u23ce": {"f1-score": 0.5816108339272986, "precision": 0.9168539325842696, "recall": 0.42588726513569936, "support": 958}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 165}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.882747068676717, "precision": 0.8754152823920266, "recall": 0.8902027027027027, "support": 1184}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 163}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.8505543237250555, "precision": 0.892093023255814, "recall": 0.8127118644067797, "support": 1180}, "\u2423": {"f1-score": 0.937637154370985, "precision": 0.90315142198309, "recall": 0.9748610304488509, "support": 12053}},
  "ppcr": 0.9786322869955157
}
```
</details>
