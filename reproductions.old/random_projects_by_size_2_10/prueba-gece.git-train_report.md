# Train report for javascript / file:///tmp/top-repos-quality-repos-r4gls_q5/prueba-gece.git HEAD b55cff353745957e4d4ef1d7aa002052ab623e56

### Classification report

PPCR: 0.858

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.995| 0.988| 0.962| 0.992| 0.978| 52736| 54156| 0.974 |
| `␣` | 0.967| 0.997| 0.909| 0.982| 0.937| 23888| 26216| 0.911 |
| `⏎␣⁺␣⁺` | 0.960| 0.976| 0.894| 0.968| 0.926| 2500| 2729| 0.916 |
| `⏎␣⁻␣⁻` | 0.975| 0.928| 0.855| 0.951| 0.911| 2465| 2677| 0.921 |
| `⏎` | 0.957| 0.965| 0.372| 0.961| 0.536| 1768| 4584| 0.386 |
| `'` | 0.992| 0.975| 0.181| 0.984| 0.307| 797| 4282| 0.186 |
| `"` | 0.958| 0.987| 0.415| 0.973| 0.579| 466| 1109| 0.420 |
| `⏎⏎` | 0.993| 0.826| 0.103| 0.902| 0.186| 334| 2683| 0.124 |
| `⏎⇥⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 84| 227| 0.370 |
| `⏎␣⁻␣⁻␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 62| 170| 0.365 |
| `⏎␣⁺␣⁺␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 58| 172| 0.337 |
| `⏎⇥⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 16| 155| 0.103 |
| `⏎⏎⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 14| 155| 0.090 |
| `macro avg` | 0.600| 0.588| 0.361| 0.593| 0.412| 85188| 99315| 0.858 |
| `micro avg` | 0.985| 0.985| 0.844| 0.985| 0.909| 85188| 99315| 0.858 |
| `weighted avg` | 0.982| 0.985| 0.844| 0.983| 0.880| 85188| 99315| 0.858 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| ⏎⇥⁺| ⏎⇥⁻| ⏎␣⁻␣⁻␣⁻␣⁻| ⏎␣⁺␣⁺␣⁺␣⁺| ⏎⏎⏎| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|1420 |52098 |638 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|2328 |33 |23822 |14 |0 |0 |15 |4 |0 |0 |0 |0 |0 |0 |
|2816 |5 |53 |1706 |0 |2 |0 |2 |0 |0 |0 |0 |0 |0 |
|3485 |0 |0 |0 |777 |0 |0 |0 |20 |0 |0 |0 |0 |0 |
|2349 |0 |6 |49 |0 |276 |0 |3 |0 |0 |0 |0 |0 |0 |
|229 |0 |59 |0 |0 |0 |2441 |0 |0 |0 |0 |0 |0 |0 |
|212 |169 |8 |0 |0 |0 |0 |2288 |0 |0 |0 |0 |0 |0 |
|643 |0 |0 |0 |6 |0 |0 |0 |460 |0 |0 |0 |0 |0 |
|143 |12 |18 |0 |0 |0 |54 |0 |0 |0 |0 |0 |0 |0 |
|139 |16 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |
|108 |12 |0 |0 |0 |0 |0 |50 |0 |0 |0 |0 |0 |0 |
|114 |2 |22 |0 |0 |0 |34 |0 |0 |0 |0 |0 |0 |0 |
|141 |0 |0 |14 |0 |0 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| __temp_static_root/bootstrap4/js/bootstrap.bundle.e52da3817494.js | 285 |
| __temp_static_root/bootstrap4/js/bootstrap.bundle.js | 285 |
| __temp_static_root/floppyforms/js/MapWidget.js | 144 |
| __temp_static_root/floppyforms/js/MapWidget.679a9c02250f.js | 144 |
| __temp_static_root/bootstrap4/js/bootstrap.bb38938b1b90.js | 134 |
| __temp_static_root/otree/js/reconnecting-websocket.js | 67 |
| __temp_static_root/otree/js/reconnecting-websocket.7e23ddc97099.js | 67 |
| __temp_static_root/otree/js/jquery.timeago.js | 43 |
| __temp_static_root/otree/js/jquery.timeago.28da9e599c5a.js | 43 |
| __temp_static_root/otree/js/jquery.animate-colors.js | 25 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.9725158562367865, "precision": 0.9583333333333334, "recall": 0.9871244635193133, "support": 466}, "\u0027": {"f1-score": 0.9835443037974685, "precision": 0.9923371647509579, "recall": 0.9749058971141782, "support": 797}, "macro avg": {"f1-score": 0.5931801310034217, "precision": 0.5997891981015563, "recall": 0.5879264329084649, "support": 85188}, "micro avg": {"f1-score": 0.984504859839414, "precision": 0.984504859839414, "recall": 0.984504859839414, "support": 85188}, "weighted avg": {"f1-score": 0.9831362887272848, "precision": 0.9820136666833523, "recall": 0.984504859839414, "support": 85188}, "\u2205": {"f1-score": 0.9915590533197569, "precision": 0.9952432804172159, "recall": 0.9879020024271845, "support": 52736}, "\u23ce": {"f1-score": 0.96085609687412, "precision": 0.9568143578238923, "recall": 0.9649321266968326, "support": 1768}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 84}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 16}, "\u23ce\u23ce": {"f1-score": 0.9019607843137254, "precision": 0.9928057553956835, "recall": 0.8263473053892215, "support": 334}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 14}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9678826328310864, "precision": 0.9595125786163522, "recall": 0.9764, "support": 2500}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 58}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9509559434746467, "precision": 0.9748615253515126, "recall": 0.9281947261663286, "support": 2465}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 62}, "\u2423": {"f1-score": 0.9820670321968916, "precision": 0.967351579631284, "recall": 0.9972371064969859, "support": 23888}},
  "cl_report_full": {"\"": {"f1-score": 0.578980490874764, "precision": 0.9583333333333334, "recall": 0.4147880973850316, "support": 1109}, "\u0027": {"f1-score": 0.3068114511352419, "precision": 0.9923371647509579, "recall": 0.18145726296123307, "support": 4282}, "macro avg": {"f1-score": 0.4123244766504086, "precision": 0.5997891981015563, "recall": 0.3608549726061774, "support": 99315}, "micro avg": {"f1-score": 0.9091234288873352, "precision": 0.984504859839414, "recall": 0.8444645823893672, "support": 99315}, "weighted avg": {"f1-score": 0.8803037857023276, "precision": 0.9751639983931946, "recall": 0.8444645823893672, "support": 99315}, "\u2205": {"f1-score": 0.9783386383482157, "precision": 0.9952432804172159, "recall": 0.961998670507423, "support": 54156}, "\u23ce": {"f1-score": 0.5358881733940631, "precision": 0.9568143578238923, "recall": 0.37216404886561955, "support": 4584}, "\u23ce\u21e5\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 227}, "\u23ce\u21e5\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 155}, "\u23ce\u23ce": {"f1-score": 0.18642350557244175, "precision": 0.9928057553956835, "recall": 0.10286992172940738, "support": 2683}, "\u23ce\u23ce\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 155}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9258486630001896, "precision": 0.9595125786163522, "recall": 0.894466837669476, "support": 2729}, "\u23ce\u2423\u207a\u2423\u207a\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 172}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9108280254777069, "precision": 0.9748615253515126, "recall": 0.8546880836757564, "support": 2677}, "\u23ce\u2423\u207b\u2423\u207b\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 170}, "\u2423": {"f1-score": 0.9370992486526888, "precision": 0.967351579631284, "recall": 0.9086817210863595, "support": 26216}},
  "ppcr": 0.8577556260383628
}
```
</details>
