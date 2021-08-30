# Test report for javascript / file:///tmp/top-repos-quality-repos-av7k39rr/open-tamil.git HEAD 9f8c5889c87ec0cee94367cf3772ab59852a142f

### Classification report

PPCR: 0.950

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.989| 0.994| 0.987| 0.991| 0.988| 54579| 54968| 0.993 |
| `␣` | 0.974| 0.975| 0.959| 0.975| 0.967| 21651| 22003| 0.984 |
| `'` | 0.836| 0.978| 0.754| 0.902| 0.793| 7821| 10142| 0.771 |
| `⏎` | 0.816| 0.936| 0.809| 0.872| 0.812| 4074| 4716| 0.864 |
| `⏎⏎` | 0.942| 0.792| 0.712| 0.861| 0.811| 2915| 3243| 0.899 |
| `⏎⇥⁻` | 0.979| 0.935| 0.838| 0.957| 0.903| 2895| 3230| 0.896 |
| `⏎⇥⁺` | 0.952| 0.883| 0.770| 0.917| 0.851| 2875| 3299| 0.871 |
| `"` | 0.887| 0.416| 0.360| 0.566| 0.512| 2522| 2914| 0.865 |
| `⏎⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 49| 50| 0.980 |
| `⏎⇥⁻⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 12| 12| 1.000 |
| `macro avg` | 0.738| 0.691| 0.619| 0.704| 0.664| 99393| 104577| 0.950 |
| `micro avg` | 0.960| 0.960| 0.912| 0.960| 0.936| 99393| 104577| 0.950 |
| `weighted avg` | 0.961| 0.960| 0.912| 0.957| 0.930| 99393| 104577| 0.950 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎| "| ⏎⏎⇥⁻| ⏎⇥⁻⇥⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|389 |54242 |230 |0 |1 |80 |26 |0 |0 |0 |0 |
|352 |190 |21110 |0 |311 |9 |1 |30 |0 |0 |0 |
|2321 |16 |8 |7650 |0 |7 |6 |0 |134 |0 |0 |
|642 |44 |119 |0 |3814 |2 |0 |95 |0 |0 |0 |
|424 |81 |149 |91 |14 |2540 |0 |0 |0 |0 |0 |
|335 |171 |11 |0 |4 |0 |2708 |1 |0 |0 |0 |
|328 |75 |1 |0 |529 |0 |0 |2310 |0 |0 |0 |
|392 |20 |18 |1405 |0 |29 |1 |0 |1049 |0 |0 |
|1 |0 |20 |0 |0 |0 |14 |15 |0 |0 |0 |
|0 |2 |0 |0 |0 |0 |10 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.5662618083670715, "precision": 0.8867286559594252, "recall": 0.4159397303727201, "support": 2522}, "\u0027": {"f1-score": 0.9017504567690222, "precision": 0.8364312267657993, "recall": 0.9781357882623706, "support": 7821}, "macro avg": {"f1-score": 0.7040538103689478, "precision": 0.7376637618165152, "recall": 0.6910431305215065, "support": 99393}, "micro avg": {"f1-score": 0.9600575493243991, "precision": 0.9600575493243991, "recall": 0.9600575493243991, "support": 99393}, "weighted avg": {"f1-score": 0.9574428112411524, "precision": 0.9608434370924948, "recall": 0.9600575493243991, "support": 99393}, "\u2205": {"f1-score": 0.9914458051544506, "precision": 0.989077514997903, "recall": 0.9938254640063028, "support": 54579}, "\u23ce": {"f1-score": 0.8720704241454212, "precision": 0.8161780440830302, "recall": 0.9361806578301424, "support": 4074}, "\u23ce\u21e5\u207a": {"f1-score": 0.9166365932876218, "precision": 0.9523809523809523, "recall": 0.8834782608695653, "support": 2875}, "\u23ce\u21e5\u207b": {"f1-score": 0.9567214273096626, "precision": 0.9790310918293564, "recall": 0.935405872193437, "support": 2895}, "\u23ce\u21e5\u207b\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u23ce\u23ce": {"f1-score": 0.8609765188222139, "precision": 0.9424724602203183, "recall": 0.7924528301886793, "support": 2915}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 49}, "\u2423": {"f1-score": 0.9746750698340143, "precision": 0.9743376719283671, "recall": 0.9750127014918479, "support": 21651}},
  "cl_report_full": {"\"": {"f1-score": 0.5120820112277277, "precision": 0.8867286559594252, "recall": 0.3599862731640357, "support": 2914}, "\u0027": {"f1-score": 0.7932393197843219, "precision": 0.8364312267657993, "recall": 0.7542890948530862, "support": 10142}, "macro avg": {"f1-score": 0.6638654614321405, "precision": 0.7376637618165152, "recall": 0.6189842323497262, "support": 104577}, "micro avg": {"f1-score": 0.9356572044908565, "precision": 0.9600575493243991, "recall": 0.9124664123086338, "support": 104577}, "weighted avg": {"f1-score": 0.9304559009253557, "precision": 0.957023881714648, "recall": 0.9124664123086338, "support": 104577}, "\u2205": {"f1-score": 0.987933593785573, "precision": 0.989077514997903, "recall": 0.9867923155290351, "support": 54968}, "\u23ce": {"f1-score": 0.8124400894663967, "precision": 0.8161780440830302, "recall": 0.8087362171331637, "support": 4716}, "\u23ce\u21e5\u207a": {"f1-score": 0.8514917867918204, "precision": 0.9523809523809523, "recall": 0.7699302819036071, "support": 3299}, "\u23ce\u21e5\u207b": {"f1-score": 0.9032688458972649, "precision": 0.9790310918293564, "recall": 0.8383900928792569, "support": 3230}, "\u23ce\u21e5\u207b\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 12}, "\u23ce\u23ce": {"f1-score": 0.8113804004214963, "precision": 0.9424724602203183, "recall": 0.7123034227567068, "support": 3243}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 50}, "\u2423": {"f1-score": 0.9668185669468043, "precision": 0.9743376719283671, "recall": 0.9594146252783712, "support": 22003}},
  "ppcr": 0.9504288705929602
}
```
</details>