# Train report for javascript / file:///tmp/top-repos-quality-repos-3nv3ivn_/django-search-movies.git HEAD 8c5de3bd573cd88d3474e468d1c3160d30079fea

### Classification report

PPCR: 0.847

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.997| 0.988| 0.964| 0.993| 0.980| 21192| 21734| 0.975 |
| `␣` | 0.964| 0.994| 0.905| 0.979| 0.934| 10005| 10990| 0.910 |
| `⏎␣⁻␣⁻` | 0.964| 0.967| 0.963| 0.965| 0.963| 1180| 1185| 0.996 |
| `⏎␣⁺␣⁺` | 0.990| 0.975| 0.952| 0.982| 0.970| 1174| 1203| 0.976 |
| `⏎` | 0.964| 0.828| 0.284| 0.891| 0.439| 650| 1895| 0.343 |
| `⏎⏎` | 0.976| 0.867| 0.103| 0.919| 0.186| 143| 1208| 0.118 |
| `'` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 1645| 0.000 |
| `"` | 0.000| 0.000| 0.000| 0.000| 0.000| 0| 693| 0.000 |
| `weighted avg` | 0.985| 0.985| 0.834| 0.985| 0.861| 34344| 40553| 0.847 |
| `micro avg` | 0.985| 0.985| 0.834| 0.985| 0.903| 34344| 40553| 0.847 |
| `macro avg` | 0.732| 0.702| 0.521| 0.716| 0.559| 34344| 40553| 0.847 |

### Confusion matrix

|refusal|  ∅| ␣| ⏎| '| ⏎⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| "| 
|:---|:---|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |0 |0 |
|542 |20942 |222 |1 |0 |0 |0 |27 |0 |
|985 |32 |9944 |6 |0 |0 |12 |11 |0 |
|1245 |2 |103 |538 |0 |2 |0 |5 |0 |
|1645 |0 |0 |0 |0 |0 |0 |0 |0 |
|1065 |0 |15 |4 |0 |124 |0 |0 |0 |
|29 |2 |26 |0 |0 |1 |1145 |0 |0 |
|5 |26 |4 |9 |0 |0 |0 |1141 |0 |
|693 |0 |0 |0 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| static/bootstrap/js/bootstrap.bundle.js | 246 |
| movies/static/movies/js/movie_detail_trailer.js | 140 |
| movies/static/movies/js/filter_list.js | 27 |
| movies/static/movies/js/filter_list_0.1.js | 26 |
| movies/static/movies/js/member.js | 20 |
| movies/static/movies/js/movie_favorite.js | 15 |
| movies/static/movies/js/votes.js | 13 |
| movies/static/movies/js/fullclip.js | 10 |
| movies/static/movies/js/movie-slider.js | 6 |
| movies/static/movies/js/paralax_bg_img.js | 5 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 0}, "macro avg": {"f1-score": 0.7160456583655535, "precision": 0.7318776314817819, "recall": 0.702397324676973, "support": 34344}, "micro avg": {"f1-score": 0.9851502445842069, "precision": 0.9851502445842069, "recall": 0.9851502445842069, "support": 34344}, "weighted avg": {"f1-score": 0.9850577593405931, "precision": 0.9853489255647505, "recall": 0.9851502445842069, "support": 34344}, "\u2205": {"f1-score": 0.9926059342117737, "precision": 0.9970481812988002, "recall": 0.9882030955077388, "support": 21192}, "\u23ce": {"f1-score": 0.890728476821192, "precision": 0.96415770609319, "recall": 0.8276923076923077, "support": 650}, "\u23ce\u23ce": {"f1-score": 0.9185185185185185, "precision": 0.9763779527559056, "recall": 0.8671328671328671, "support": 143}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9824109824109825, "precision": 0.989628349178911, "recall": 0.975298126064736, "support": 1174}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9653130287648054, "precision": 0.9636824324324325, "recall": 0.9669491525423729, "support": 1180}, "\u2423": {"f1-score": 0.9787883261971554, "precision": 0.9641264300950165, "recall": 0.9939030484757622, "support": 10005}},
  "cl_report_full": {"\"": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 693}, "\u0027": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 1645}, "macro avg": {"f1-score": 0.5589475454705558, "precision": 0.7318776314817819, "recall": 0.521199047848103, "support": 40553}, "micro avg": {"f1-score": 0.903480780271573, "precision": 0.9851502445842069, "recall": 0.8343155870095924, "support": 40553}, "weighted avg": {"f1-score": 0.8611861760585199, "precision": 0.927295752362655, "recall": 0.8343155870095924, "support": 40553}, "\u2205": {"f1-score": 0.9800177827694323, "precision": 0.9970481812988002, "recall": 0.9635594000184043, "support": 21734}, "\u23ce": {"f1-score": 0.4386465552384835, "precision": 0.96415770609319, "recall": 0.28390501319261213, "support": 1895}, "\u23ce\u23ce": {"f1-score": 0.1857677902621723, "precision": 0.9763779527559056, "recall": 0.10264900662251655, "support": 1208}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.9703389830508474, "precision": 0.989628349178911, "recall": 0.9517871986699917, "support": 1203}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.9632756437315322, "precision": 0.9636824324324325, "recall": 0.9628691983122363, "support": 1185}, "\u2423": {"f1-score": 0.933533608711979, "precision": 0.9641264300950165, "recall": 0.9048225659690627, "support": 10990}},
  "ppcr": 0.8468917219441225
}
```
</details>
