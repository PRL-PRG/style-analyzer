# Model report for file:///tmp/top-repos-quality-repos-8df8oc23/formula.js.git HEAD f52b7efd424036fdd708c9de3ac501f41d595d8c

### Dump

```json
{'created_at': '2021-08-29 11:33:25',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.15.0-135-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '17.8 kB',
 'tags': [],
 'uuid': '5ab5d542-ea7b-41d6-8d8a-1cbd4d84c225',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-8df8oc23/formula.js.git f52b7efd424036fdd708c9de3ac501f41d595d8c

# javascript
43 rules, avg.len. 8.2
## train
PPCR: 0.982516
### report
macro
{'f1-score': 0.9533814838501091,
 'precision': 0.9744178498485621,
 'recall': 0.9355337652349518,
 'support': 79233}
micro
{'f1-score': 0.9825577726452362,
 'precision': 0.9825577726452362,
 'recall': 0.9825577726452362,
 'support': 79233}
weighted
{'f1-score': 0.9824241054897965,
 'precision': 0.9828876116652949,
 'recall': 0.9825577726452362,
 'support': 79233}
### report_full
macro
{'f1-score': 0.9271838994882788,
 'precision': 0.9744178498485621,
 'recall': 0.8880551852766176,
 'support': 80643}
micro
{'f1-score': 0.9738922665065425,
 'precision': 0.9825577726452362,
 'recall': 0.9653782721376933,
 'support': 80643}
weighted
{'f1-score': 0.973237508911937,
 'precision': 0.9825924446601202,
 'recall': 0.9653782721376933,
 'support': 80643}
## test
PPCR: 0.977237
### report
macro
{'f1-score': 0.7952636032821304,
 'precision': 0.8077149719923318,
 'recall': 0.7965641230310503,
 'support': 16700}
micro
{'f1-score': 0.9697604790419162,
 'precision': 0.9697604790419162,
 'recall': 0.9697604790419162,
 'support': 16700}
weighted
{'f1-score': 0.9672137160205249,
 'precision': 0.9666396922862757,
 'recall': 0.9697604790419162,
 'support': 16700}
### report_full
macro
{'f1-score': 0.7809735987583254,
 'precision': 0.8077149719923318,
 'recall': 0.769125738600787,
 'support': 17089}
micro
{'f1-score': 0.958595992778715,
 'precision': 0.9697604790419162,
 'recall': 0.9476856457370239,
 'support': 17089}
weighted
{'f1-score': 0.9558786008253817,
 'precision': 0.9665341110559557,
 'recall': 0.9476856457370239,
 'support': 17089}
```

## javascript
### Summary
37 rules, avg.len. 8.1

| | |
|-|-|
|Min support|91|
|Max support|13668|
|Min confidence|0.9210526347160339|
|Max confidence|0.9998810291290283|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 90,
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.roles in {STRING}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.992. Support: 315.` |
| 2 | `  -1.roles not in {STRING}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 13668.` |
| 3 | `  -1.reserved = (<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 566.` |
| 4 | `  -1.reserved not in {(}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 336.` |
| 5 | `  •••start_col ≥ 24<br>	∧ -1.reserved not in {(}<br>	∧ -3.reserved = =<br>	∧ +1.reserved not in {)}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.921. Support: 285.` |
| 6 | `  -1.reserved not in {(}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 139.` |
| 7 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {), ]}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 6129.` |
| 8 | `  •••start_line ≥ 254<br>	∧ -1.roles in {EXPRESSION, STRING}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type = ArrayExpression<br>⇒ y = "<br>Confidence: 0.999. Support: 406.` |
| 9 | `  •••start_line ≤ 254<br>	∧ -1.roles in {EXPRESSION, STRING}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.958. Support: 1189.` |
| 10 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 590.` |
| 11 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 108.` |
| 12 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 11858.` |
| 13 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.942. Support: 778.` |
| 14 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 4203.` |
| 15 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2752.` |
| 16 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.998. Support: 1219.` |
| 17 | `  •••start_col ≥ 7<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ ^1.roles in {SCOPE} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.969. Support: 1782.` |
| 18 | `  •••start_col ≥ 7<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ ^1.internal_type = ForStatement<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED, SCOPE}<br>⇒ y = ␣<br>Confidence: 0.987. Support: 192.` |
| 19 | `  •••start_col ≤ 6<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.958. Support: 558.` |
| 20 | `  -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.988. Support: 1292.` |
| 21 | `  •••start_line ≥ 187<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved = ,<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = "<br>Confidence: 0.990. Support: 149.` |
| 22 | `  •••start_line ≤ 187<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved = ,<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.997. Support: 187.` |
| 23 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved = ,<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 1105.` |
| 24 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 733.` |
| 25 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved = ,<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved not in {), ;}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles in {LIST}<br>⇒ y = "<br>Confidence: 0.986. Support: 245.` |
| 26 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved = ,<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved not in {), ;}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles not in {LIST}<br>⇒ y = '<br>Confidence: 0.926. Support: 576.` |
| 27 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {,}<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 424.` |
| 28 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {,}<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved not in {), ,, ;}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.943. Support: 414.` |
| 29 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {,}<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved not in {), ,}<br>	∧ +2.roles in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.980. Support: 177.` |
| 30 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.995. Support: 97.` |
| 31 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {,}<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved not in {), ,}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 15<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 364.` |
| 32 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved = ]<br>	∧ -4.reserved not in {.}<br>	∧ +1.reserved not in {), ;}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles in {VARIABLE}<br>⇒ y = ⏎<br>Confidence: 0.997. Support: 158.` |
| 33 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved = ]<br>	∧ -4.reserved not in {.}<br>	∧ +1.length ≤ 14<br>	∧ +2.internal_type = StringLiteral<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 128.` |
| 34 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {,}<br>	∧ -3.reserved not in {,}<br>	∧ -3.length ≥ 2<br>	∧ -4.reserved not in {.}<br>	∧ +1.reserved not in {,}<br>	∧ +1.length ≤ 14<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles in {ARGUMENT}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 106.` |
| 35 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {,, ]}<br>	∧ -2.roles in {STRING}<br>	∧ -3.reserved not in {,}<br>	∧ -3.length ≤ 2<br>	∧ -4.reserved not in {.}<br>	∧ +1.reserved not in {), ,, ;}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles in {ARGUMENT}<br>⇒ y = '<br>Confidence: 0.966. Support: 103.` |
| 36 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {,, ]}<br>	∧ -3.reserved not in {,}<br>	∧ -3.length ≤ 2<br>	∧ -4.reserved not in {.}<br>	∧ +1.reserved not in {), ,, ;}<br>	∧ +1.length ≤ 14<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles in {ARGUMENT}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 91.` |
| 37 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {,, ]}<br>	∧ -3.reserved not in {,}<br>	∧ -4.reserved not in {.}<br>	∧ +1.reserved not in {), ,, ;}<br>	∧ +1.length ≤ 14<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles not in {ARGUMENT}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 8907.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.08108108108108, "max_conf": 0.9998810291290283, "max_support": 13668, "min_conf": 0.9210526347160339, "min_support": 91, "num_rules": 37}}
```
</details>
