# Test report for javascript / file:///tmp/top-repos-quality-repos-ke1ue5c3/demo.git HEAD 2511ba1002cf1dc3be40d2ad647e428c48802bc0

### Classification report

PPCR: 0.841

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.964| 0.977| 0.920| 0.971| 0.942| 6746| 7165| 0.942 |
| `␣` | 0.899| 0.929| 0.737| 0.914| 0.810| 2301| 2899| 0.794 |
| `'` | 0.966| 0.985| 0.873| 0.975| 0.917| 1236| 1394| 0.887 |
| `⏎` | 0.902| 0.921| 0.522| 0.912| 0.661| 522| 922| 0.566 |
| `⏎␣⁺␣⁺` | 0.895| 0.667| 0.399| 0.764| 0.552| 192| 321| 0.598 |
| `"` | 0.985| 0.719| 0.686| 0.831| 0.809| 185| 194| 0.954 |
| `⏎␣⁻␣⁻` | 0.943| 0.806| 0.369| 0.869| 0.531| 144| 314| 0.459 |
| `⏎⇥⁺` | 0.810| 0.935| 0.564| 0.868| 0.665| 123| 204| 0.603 |
| `⏎⇥⁻` | 1.000| 0.582| 0.348| 0.736| 0.516| 122| 204| 0.598 |
| `⏎⏎` | 0.962| 0.490| 0.142| 0.649| 0.248| 51| 176| 0.290 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 22| 25| 0.880 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 3| 29| 0.103 |
| `macro avg` | 0.777| 0.668| 0.463| 0.707| 0.554| 11647| 13847| 0.841 |
| `micro avg` | 0.946| 0.946| 0.796| 0.946| 0.864| 11647| 13847| 0.841 |
| `weighted avg` | 0.944| 0.946| 0.796| 0.943| 0.850| 11647| 13847| 0.841 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| ⏎⇥⁺| ⏎⇥⁻| "| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎␣⁻␣⁻␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|419 |6592 |140 |0 |6 |2 |1 |0 |5 |0 |0 |0 |0 |
|598 |120 |2138 |3 |25 |1 |0 |0 |14 |0 |0 |0 |0 |
|158 |2 |15 |1217 |0 |0 |0 |0 |0 |0 |2 |0 |0 |
|400 |13 |27 |0 |481 |0 |0 |1 |0 |0 |0 |0 |0 |
|129 |35 |23 |1 |0 |128 |0 |0 |5 |0 |0 |0 |0 |
|170 |20 |2 |0 |6 |0 |116 |0 |0 |0 |0 |0 |0 |
|125 |5 |6 |0 |15 |0 |0 |25 |0 |0 |0 |0 |0 |
|81 |7 |0 |0 |0 |1 |0 |0 |115 |0 |0 |0 |0 |
|82 |38 |7 |0 |0 |0 |6 |0 |0 |71 |0 |0 |0 |
|9 |0 |13 |39 |0 |0 |0 |0 |0 |0 |133 |0 |0 |
|3 |3 |5 |0 |0 |11 |0 |0 |3 |0 |0 |0 |0 |
|26 |1 |2 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.83125, "precision": 0.9851851851851852, "recall": 0.7189189189189189, "support": 185}, "\u0027": {"f1-score": 0.9751602564102565, "precision": 0.9658730158730159, "recall": 0.9846278317152104, "support": 1236}, "macro avg": {"f1-score": 0.7074120856683833, "precision": 0.7772058860655847, "recall": 0.6675567036848049, "support": 11647}, "micro avg": {"f1-score": 0.9458229587018117, "precision": 0.9458229587018117, "recall": 0.9458229587018117, "support": 11647}, "weighted avg": {"f1-score": 0.9433912635106615, "precision": 0.9444021013447189, "recall": 0.9458229587018117, "support": 11647}, "\u2205": {"f1-score": 0.9706965100868797, "precision": 0.9643066120538326, "recall": 0.9771716572783872, "support": 6746}, "\u23ce": {"f1-score": 0.9118483412322275, "precision": 0.9024390243902439, "recall": 0.921455938697318, "support": 522}, "\u23ce\u21e5\u207a": {"f1-score": 0.8679245283018868, "precision": 0.8098591549295775, "recall": 0.9349593495934959, "support": 123}, "\u23ce\u21e5\u207b": {"f1-score": 0.7357512953367876, "precision": 1.0, "recall": 0.5819672131147541, "support": 122}, "\u23ce\u23ce": {"f1-score": 0.6493506493506493, "precision": 0.9615384615384616, "recall": 0.49019607843137253, "support": 51}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.764179104477612, "precision": 0.8951048951048951, "recall": 0.6666666666666666, "support": 192}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 22}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8689138576779026, "precision": 0.943089430894309, "recall": 0.8055555555555556, "support": 144}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 3}, "\u2423": {"f1-score": 0.9138704851463987, "precision": 0.8990748528174937, "recall": 0.92916123424598, "support": 2301}},
  "cl_report_full": {"\"": {"f1-score": 0.8085106382978724, "precision": 0.9851851851851852, "recall": 0.6855670103092784, "support": 194}, "\u0027": {"f1-score": 0.9171062547098718, "precision": 0.9658730158730159, "recall": 0.8730272596843616, "support": 1394}, "macro avg": {"f1-score": 0.5541655042000889, "precision": 0.7772058860655847, "recall": 0.46331672097508453, "support": 13847}, "micro avg": {"f1-score": 0.8642033419628149, "precision": 0.9458229587018117, "recall": 0.7955513829710407, "support": 13847}, "weighted avg": {"f1-score": 0.8499454023548195, "precision": 0.9393498486091015, "recall": 0.7955513829710407, "support": 13847}, "\u2205": {"f1-score": 0.9416470252124848, "precision": 0.9643066120538326, "recall": 0.9200279134682484, "support": 7165}, "\u23ce": {"f1-score": 0.6611683848797251, "precision": 0.9024390243902439, "recall": 0.5216919739696312, "support": 922}, "\u23ce\u21e5\u207a": {"f1-score": 0.6647398843930635, "precision": 0.8098591549295775, "recall": 0.5637254901960784, "support": 204}, "\u23ce\u21e5\u207b": {"f1-score": 0.5163636363636364, "precision": 1.0, "recall": 0.3480392156862745, "support": 204}, "\u23ce\u23ce": {"f1-score": 0.24752475247524755, "precision": 0.9615384615384616, "recall": 0.14204545454545456, "support": 176}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.5517241379310345, "precision": 0.8951048951048951, "recall": 0.3987538940809969, "support": 321}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 25}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.5308924485125858, "precision": 0.943089430894309, "recall": 0.36942675159235666, "support": 314}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 29}, "\u2423": {"f1-score": 0.8103088876255449, "precision": 0.8990748528174937, "recall": 0.7374956881683339, "support": 2899}},
  "ppcr": 0.8411208203943092
}
```
</details>