# Test report for javascript / file:///tmp/top-repos-quality-repos-3r68gtq8/openbroadcast.org.git HEAD 05f12d860939331178353ef94950c5e031ae0cfd

### Classification report

PPCR: 0.917

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.979| 0.943| 0.936| 0.961| 0.957| 56126| 56510| 0.993 |
| `␣` | 0.849| 0.953| 0.921| 0.898| 0.883| 24880| 25748| 0.966 |
| `'` | 0.898| 0.958| 0.812| 0.927| 0.853| 4690| 5534| 0.847 |
| `⏎` | 0.748| 0.835| 0.674| 0.789| 0.709| 4181| 5179| 0.807 |
| `⏎⏎` | 0.777| 0.386| 0.248| 0.516| 0.376| 1221| 1899| 0.643 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.914| 0.789| 0.267| 0.847| 0.414| 825| 2434| 0.339 |
| `"` | 0.744| 0.204| 0.159| 0.320| 0.262| 658| 844| 0.780 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.863| 0.344| 0.068| 0.492| 0.126| 532| 2684| 0.198 |
| `⏎␣⁻␣⁻` | 0.947| 0.321| 0.088| 0.480| 0.161| 56| 204| 0.275 |
| `⏎⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 55| 129| 0.426 |
| `⏎⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 41| 108| 0.380 |
| `␣␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 41| 49| 0.837 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 38| 230| 0.165 |
| `⏎⇥⁻` | 0.621| 0.562| 0.207| 0.590| 0.310| 32| 87| 0.368 |
| `⏎⇥⁺` | 0.245| 0.414| 0.130| 0.308| 0.170| 29| 92| 0.315 |
| `⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 25| 98| 0.255 |
| `⏎⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 18| 0.667 |
| `⏎⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 5| 97| 0.052 |
| `⏎⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 4| 13| 0.308 |
| `⇥` | 0.000| 0.000| 0.000| 0.000| 0.000| 2| 4| 0.500 |
| `⏎⇥⁻⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 1| 1| 1.000 |
| `⇥⇥` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 0| 0.000 |
| `⏎⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 3| 0.000 |
| `macro avg` | 0.373| 0.292| 0.196| 0.310| 0.227| 93454| 101965| 0.917 |
| `micro avg` | 0.921| 0.921| 0.844| 0.921| 0.881| 93454| 101965| 0.917 |
| `weighted avg` | 0.922| 0.921| 0.844| 0.918| 0.859| 93454| 101965| 0.917 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⏎| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| "| ⏎⇥⁺| ⏎⇥⁻| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎␣⁻␣⁻␣⁻␣⁻| ⏎⏎⏎| ⏎⏎␣⁺␣⁺␣⁺␣⁺| ⏎⏎⇥⁻| ⏎⏎⇥⁺| ⇥| ␣␣| ⏎␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻␣⁻| ⏎⏎␣⁻␣⁻| ⏎⇥⁻⇥⁻| ⇥⇥| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|384 |52920 |3160 |40 |0 |1 |3 |0 |0 |0 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|868 |938 |23708 |188 |3 |8 |25 |3 |0 |6 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|998 |35 |547 |3490 |0 |102 |0 |6 |0 |0 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|844 |96 |20 |34 |4494 |0 |0 |0 |46 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|678 |3 |14 |732 |0 |471 |0 |0 |0 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2152 |29 |263 |26 |4 |0 |183 |0 |0 |27 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1609 |15 |96 |58 |0 |0 |0 |651 |0 |0 |4 |0 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|186 |3 |7 |14 |500 |0 |0 |0 |134 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|63 |0 |12 |5 |0 |0 |0 |0 |0 |12 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|55 |0 |0 |13 |0 |0 |0 |1 |0 |0 |18 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|192 |6 |29 |1 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|148 |2 |21 |10 |0 |0 |0 |5 |0 |0 |0 |0 |18 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|74 |0 |2 |20 |0 |3 |0 |30 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|67 |2 |3 |19 |0 |17 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|92 |0 |0 |3 |0 |0 |1 |0 |0 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|6 |0 |0 |7 |0 |2 |0 |0 |0 |0 |3 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|9 |0 |0 |1 |0 |1 |0 |0 |0 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2 |0 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|8 |4 |33 |3 |0 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|73 |3 |4 |2 |0 |0 |0 |16 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|3 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|0 |0 |0 |1 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.3198090692124105, "precision": 0.7444444444444445, "recall": 0.20364741641337386, "support": 658}, "\u0027": {"f1-score": 0.92726709996905, "precision": 0.8982610433739756, "recall": 0.9582089552238806, "support": 4690}, "macro avg": {"f1-score": 0.3098720260877152, "precision": 0.3733183091137821, "recall": 0.2916914892000763, "support": 93454}, "micro avg": {"f1-score": 0.9212981787831447, "precision": 0.9212981787831447, "recall": 0.9212981787831447, "support": 93454}, "weighted avg": {"f1-score": 0.9176622693492074, "precision": 0.9217814980837237, "recall": 0.9212981787831447, "support": 93454}, "\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2}, "\u21e5\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2205": {"f1-score": 0.9605924742698444, "precision": 0.9789847565487643, "recall": 0.9428785233225243, "support": 56126}, "\u23ce": {"f1-score": 0.788878842676311, "precision": 0.7478037283051211, "recall": 0.8347285338435781, "support": 4181}, "\u23ce\u21e5\u207a": {"f1-score": 0.3076923076923077, "precision": 0.24489795918367346, "recall": 0.41379310344827586, "support": 29}, "\u23ce\u21e5\u207b": {"f1-score": 0.5901639344262296, "precision": 0.6206896551724138, "recall": 0.5625, "support": 32}, "\u23ce\u21e5\u207b\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u23ce": {"f1-score": 0.5155993431855501, "precision": 0.7772277227722773, "recall": 0.3857493857493858, "support": 1221}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 41}, "\u23ce\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 5}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 55}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 38}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.4919354838709677, "precision": 0.8632075471698113, "recall": 0.34398496240601506, "support": 532}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.48, "precision": 0.9473684210526315, "recall": 0.32142857142857145, "support": 56}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.8471047495120364, "precision": 0.9143258426966292, "recall": 0.7890909090909091, "support": 825}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 25}, "\u2423": {"f1-score": 0.8980132952027423, "precision": 0.8491099888972458, "recall": 0.9528938906752411, "support": 24880}, "\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 41}},
  "cl_report_full": {"\"": {"f1-score": 0.26171875, "precision": 0.7444444444444445, "recall": 0.15876777251184834, "support": 844}, "\u0027": {"f1-score": 0.8529942108759608, "precision": 0.8982610433739756, "recall": 0.812070834839176, "support": 5534}, "macro avg": {"f1-score": 0.22707202974016158, "precision": 0.3733183091137821, "recall": 0.1961387187743376, "support": 101965}, "micro avg": {"f1-score": 0.8811732738372421, "precision": 0.9212981787831447, "recall": 0.8443975874074438, "support": 101965}, "weighted avg": {"f1-score": 0.8590372428604157, "precision": 0.9115436001975523, "recall": 0.8443975874074438, "support": 101965}, "\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 4}, "\u21e5\u21e5": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u2205": {"f1-score": 0.9572562994048802, "precision": 0.9789847565487643, "recall": 0.9364714209874359, "support": 56510}, "\u23ce": {"f1-score": 0.7089173268332318, "precision": 0.7478037283051211, "recall": 0.6738752654952693, "support": 5179}, "\u23ce\u21e5\u207a": {"f1-score": 0.1702127659574468, "precision": 0.24489795918367346, "recall": 0.13043478260869565, "support": 92}, "\u23ce\u21e5\u207b": {"f1-score": 0.3103448275862069, "precision": 0.6206896551724138, "recall": 0.20689655172413793, "support": 87}, "\u23ce\u21e5\u207b\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1}, "\u23ce\u23ce": {"f1-score": 0.37604790419161677, "precision": 0.7772277227722773, "recall": 0.2480252764612954, "support": 1899}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 13}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 18}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 108}, "\u23ce\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 97}, "\u23ce\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u23ce\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 129}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 230}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.12638121546961326, "precision": 0.8632075471698113, "recall": 0.06818181818181818, "support": 2684}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.16143497757847536, "precision": 0.9473684210526315, "recall": 0.08823529411764706, "support": 204}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.41385886840432295, "precision": 0.9143258426966292, "recall": 0.2674609695973706, "support": 2434}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 98}, "\u2423": {"f1-score": 0.8834895377219624, "precision": 0.8491099888972458, "recall": 0.9207705452850707, "support": 25748}, "\u2423\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 49}},
  "ppcr": 0.9165301819251704
}
```
</details>