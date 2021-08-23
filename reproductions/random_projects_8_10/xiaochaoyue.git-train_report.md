# Train report for javascript / file:///tmp/top-repos-quality-repos-5avqt3ay/xiaochaoyue.git HEAD 5a5bc6959257a67a0aac97ccf895b9755310401e

### Classification report

PPCR: 0.844

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `␣` | 0.947| 0.993| 0.878| 0.969| 0.911| 30720| 34760| 0.884 |
| `∅` | 0.992| 0.965| 0.945| 0.978| 0.968| 26592| 27160| 0.979 |
| `"` | 1.000| 1.000| 0.924| 1.000| 0.960| 2940| 3182| 0.924 |
| `⏎` | 0.993| 0.863| 0.538| 0.923| 0.697| 2766| 4438| 0.623 |
| `⏎⇥⁺` | 0.936| 0.872| 0.767| 0.903| 0.843| 2604| 2960| 0.880 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 96| 2690| 0.036 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 80| 1998| 0.040 |
| `⏎⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 38| 378| 0.101 |
| `⏎⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 6| 406| 0.015 |
| `macro avg` | 0.541| 0.521| 0.450| 0.530| 0.487| 65842| 77972| 0.844 |
| `micro avg` | 0.968| 0.968| 0.818| 0.968| 0.887| 65842| 77972| 0.844 |
| `weighted avg` | 0.966| 0.968| 0.818| 0.967| 0.854| 65842| 77972| 0.844 |

### Confusion matrix

|refusal|  ␣| ∅| ⏎| "| ⏎⇥⁺| ⏎⇥⁻| ⏎⏎| ⏎⏎⇥⁻| ⏎⏎⇥⁺| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|4040 |30502 |174 |0 |0 |44 |0 |0 |0 |0 |
|568 |934 |25658 |0 |0 |0 |0 |0 |0 |0 |
|1672 |358 |8 |2386 |0 |14 |0 |0 |0 |0 |
|242 |0 |0 |0 |2940 |0 |0 |0 |0 |0 |
|356 |328 |4 |2 |0 |2270 |0 |0 |0 |0 |
|2594 |82 |10 |0 |0 |4 |0 |0 |0 |0 |
|1918 |14 |0 |16 |0 |50 |0 |0 |0 |0 |
|400 |0 |0 |0 |0 |6 |0 |0 |0 |0 |
|340 |0 |0 |0 |0 |38 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| Indepandent/packages/jQuery.3.3.1/Content/Scripts/jquery-3.3.1.slim.js | 1043 |
| Indepandent/Indepandent/Scripts/jquery-3.3.1.slim.js | 1043 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 2940}, "macro avg": {"f1-score": 0.5303696973862068, "precision": 0.5408184350678839, "recall": 0.5213481766388078, "support": 65842}, "micro avg": {"f1-score": 0.9683180948330853, "precision": 0.9683180948330853, "recall": 0.9683180948330853, "support": 65842}, "weighted avg": {"f1-score": 0.9665329344333483, "precision": 0.9658885729067953, "recall": 0.9683180948330853, "support": 65842}, "\u2205": {"f1-score": 0.9784540289059224, "precision": 0.9924189680513653, "recall": 0.9648766546329723, "support": 26592}, "\u23ce": {"f1-score": 0.923017408123791, "precision": 0.9925124792013311, "recall": 0.8626174981923355, "support": 2766}, "\u23ce\u21e5\u207a": {"f1-score": 0.9025844930417495, "precision": 0.9356966199505359, "recall": 0.8717357910906298, "support": 2604}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 96}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 80}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 38}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 6}, "\u2423": {"f1-score": 0.969271346404398, "precision": 0.9467378484077223, "recall": 0.9929036458333333, "support": 30720}},
  "cl_report_full": {"\"": {"f1-score": 0.9604704344985299, "precision": 1.0, "recall": 0.9239472030169704, "support": 3182}, "macro avg": {"f1-score": 0.4866256312632617, "precision": 0.5408184350678839, "recall": 0.4500744022294132, "support": 77972}, "micro avg": {"f1-score": 0.8866452501147316, "precision": 0.9683180948330853, "recall": 0.8176781408710819, "support": 77972}, "weighted avg": {"f1-score": 0.8541054715899955, "precision": 0.900568654910437, "recall": 0.8176781408710819, "support": 77972}, "\u2205": {"f1-score": 0.9679707247142264, "precision": 0.9924189680513653, "recall": 0.9446980854197349, "support": 27160}, "\u23ce": {"f1-score": 0.6974568839520608, "precision": 0.9925124792013311, "recall": 0.5376295628661559, "support": 4438}, "\u23ce\u21e5\u207a": {"f1-score": 0.8429261047159302, "precision": 0.9356966199505359, "recall": 0.7668918918918919, "support": 2960}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2690}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1998}, "\u23ce\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 378}, "\u23ce\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 406}, "\u2423": {"f1-score": 0.9108065334886083, "precision": 0.9467378484077223, "recall": 0.8775028768699655, "support": 34760}},
  "ppcr": 0.8444313343251424
}
```
</details>
