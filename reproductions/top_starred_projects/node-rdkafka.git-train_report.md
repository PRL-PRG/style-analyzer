# Train report for javascript / file:///tmp/top-repos-quality-repos-tet1vdcc/node-rdkafka.git HEAD 3ae6d726a4505ed9ef14c99f67c71a479b5c47d1

### Classification report

PPCR: 0.821

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.987| 0.990| 0.941| 0.989| 0.964| 17992| 18922| 0.951 |
| `␣` | 0.963| 0.973| 0.856| 0.968| 0.906| 6889| 7829| 0.880 |
| `'` | 1.000| 1.000| 0.499| 1.000| 0.665| 1317| 2641| 0.499 |
| `⏎␣⁺␣⁺` | 0.954| 0.973| 0.921| 0.963| 0.937| 1203| 1271| 0.946 |
| `⏎␣⁻␣⁻` | 0.994| 0.933| 0.699| 0.963| 0.821| 913| 1219| 0.749 |
| `⏎` | 0.948| 0.831| 0.110| 0.886| 0.198| 266| 2004| 0.133 |
| `⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 48| 1001| 0.048 |
| `weighted avg` | 0.979| 0.981| 0.805| 0.980| 0.851| 28628| 34887| 0.821 |
| `micro avg` | 0.981| 0.981| 0.805| 0.981| 0.884| 28628| 34887| 0.821 |
| `macro avg` | 0.835| 0.814| 0.575| 0.824| 0.642| 28628| 34887| 0.821 |

### Confusion matrix

|refusal|  ∅| ␣| '| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| ⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |
|930 |17806 |182 |0 |0 |4 |0 |0 |
|940 |129 |6704 |0 |0 |51 |5 |0 |
|1324 |0 |0 |1317 |0 |0 |0 |0 |
|1738 |42 |3 |0 |221 |0 |0 |0 |
|68 |12 |21 |0 |0 |1170 |0 |0 |
|306 |7 |52 |0 |0 |2 |852 |0 |
|953 |36 |0 |0 |12 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| e2e/both.spec.js | 55 |
| e2e/consumer.spec.js | 51 |
| test/producer/high-level-producer.spec.js | 47 |
| lib/client.js | 31 |
| util/test-producer-delivery.js | 28 |
| ci/update-version.js | 27 |
| test/producer-stream.spec.js | 27 |
| lib/kafka-consumer-stream.js | 24 |
| lib/kafka-consumer.js | 18 |
| ci/checks/librdkafka-correct-version.js | 17 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\u0027": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 1317}, "macro avg": {"f1-score": 0.8240037571787474, "precision": 0.8352310271255056, "recall": 0.8141986581876826, "support": 28628}, "micro avg": {"f1-score": 0.980508592985888, "precision": 0.980508592985888, "recall": 0.980508592985888, "support": 28628}, "weighted avg": {"f1-score": 0.9796322387921935, "precision": 0.9789120864386959, "recall": 0.980508592985888, "support": 28628}, "\u2205": {"f1-score": 0.9885631801021542, "precision": 0.9874667258207631, "recall": 0.9896620720320143, "support": 17992}, "\u23ce": {"f1-score": 0.8857715430861722, "precision": 0.9484978540772532, "recall": 0.8308270676691729, "support": 266}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 48}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.962962962962963, "precision": 0.9535452322738386, "recall": 0.972568578553616, "support": 1203}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9627118644067797, "precision": 0.9941656942823804, "recall": 0.9331872946330778, "support": 913}, "\u2423": {"f1-score": 0.968016749693163, "precision": 0.9629416834243033, "recall": 0.9731455944258963, "support": 6889}},
  "cl_report_full": {"\u0027": {"f1-score": 0.6654876200101061, "precision": 1.0, "recall": 0.4986747444149943, "support": 2641}, "macro avg": {"f1-score": 0.6415448708434021, "precision": 0.8352310271255056, "recall": 0.5751067528827308, "support": 34887}, "micro avg": {"f1-score": 0.8838856962922145, "precision": 0.980508592985888, "recall": 0.8045977011494253, "support": 34887}, "weighted avg": {"f1-score": 0.8506462690144286, "precision": 0.9513385930756567, "recall": 0.8045977011494253, "support": 34887}, "\u2205": {"f1-score": 0.9636845808302213, "precision": 0.9874667258207631, "recall": 0.941021033717366, "support": 18922}, "\u23ce": {"f1-score": 0.1975860527492177, "precision": 0.9484978540772532, "recall": 0.11027944111776447, "support": 2004}, "\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1001}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9367493995196157, "precision": 0.9535452322738386, "recall": 0.9205350118017309, "support": 1271}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.8208092485549133, "precision": 0.9941656942823804, "recall": 0.6989335520918786, "support": 1219}, "\u2423": {"f1-score": 0.9064971942397404, "precision": 0.9629416834243033, "recall": 0.8563034870353813, "support": 7829}},
  "ppcr": 0.8205921976667526
}
```
</details>
