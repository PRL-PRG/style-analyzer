# Model report for file:///tmp/top-repos-quality-repos-7z_5arwa/cursos.git HEAD f78a147444d202ce2f1468097593a8877b7500cc

### Dump

```json
{'created_at': '2021-08-21 09:16:17',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-81-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '17.3 kB',
 'tags': [],
 'uuid': '96e1f9b7-f107-49f4-9c86-f33668f8f9f3',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-7z_5arwa/cursos.git f78a147444d202ce2f1468097593a8877b7500cc

# javascript
27 rules, avg.len. 8.2
## train
PPCR: 0.919976
### report
macro
{'f1-score': 0.8198968317925578,
 'precision': 0.8370963835609231,
 'recall': 0.8120777351326607,
 'support': 32523}
micro
{'f1-score': 0.9743566091688959,
 'precision': 0.9743566091688959,
 'recall': 0.9743566091688959,
 'support': 32523}
weighted
{'f1-score': 0.9714582758326811,
 'precision': 0.9698851075315716,
 'recall': 0.9743566091688959,
 'support': 32523}
### report_full
macro
{'f1-score': 0.7109383511432368,
 'precision': 0.8370963835609231,
 'recall': 0.668907738411975,
 'support': 35352}
micro
{'f1-score': 0.9337458563535911,
 'precision': 0.9743566091688959,
 'recall': 0.8963849287169042,
 'support': 35352}
weighted
{'f1-score': 0.9151799046767244,
 'precision': 0.9632847594120872,
 'recall': 0.8963849287169042,
 'support': 35352}
## test
PPCR: 0.899287
### report
macro
{'f1-score': 0.6091351133790914,
 'precision': 0.6589706762302813,
 'recall': 0.5856515223693097,
 'support': 1893}
micro
{'f1-score': 0.90121500264131,
 'precision': 0.90121500264131,
 'recall': 0.90121500264131,
 'support': 1893}
weighted
{'f1-score': 0.898585502131555,
 'precision': 0.9063883896135413,
 'recall': 0.90121500264131,
 'support': 1893}
### report_full
macro
{'f1-score': 0.5714277443237685,
 'precision': 0.6589706762302813,
 'recall': 0.530576776387573,
 'support': 2105}
micro
{'f1-score': 0.8534267133566783,
 'precision': 0.90121500264131,
 'recall': 0.8104513064133017,
 'support': 2105}
weighted
{'f1-score': 0.843138356301571,
 'precision': 0.8972406182374861,
 'recall': 0.8104513064133017,
 'support': 2105}
```

## javascript
### Summary
19 rules, avg.len. 8.5

| | |
|-|-|
|Min support|104|
|Max support|5968|
|Min confidence|0.9460573792457581|
|Max confidence|0.9997487664222717|

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
| 1 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 5968.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 104.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.reserved not in {]}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.946. Support: 3031.` |
| 4 | `  -1.reserved = {<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.992. Support: 859.` |
| 5 | `  -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.999. Support: 585.` |
| 6 | `  -1.reserved not in {;, {}<br>	∧ -3.diff_col ≥ 4<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.997. Support: 1459.` |
| 7 | `  -1.reserved not in {;, {}<br>	∧ -3.diff_col ≤ 3<br>	∧ +3.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = '<br>Confidence: 0.974. Support: 135.` |
| 8 | `  -1.reserved not in {;, {}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 3764.` |
| 9 | `  -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1990.` |
| 10 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = '<br>Confidence: 0.970. Support: 184.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 1279.` |
| 12 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 407.` |
| 13 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 372.` |
| 14 | `  -1.diff_col ≥ 9<br>	∧ -1.diff_offset ≥ 103<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 122.` |
| 15 | `  -1.diff_col ≤ 8<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.997. Support: 145.` |
| 16 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles in {KEY} and not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.988. Support: 213.` |
| 17 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 191.` |
| 18 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -2.diff_offset ≥ 16<br>	∧ -2.internal_type = Identifier<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {), ,, ;}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.length ≤ 13<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 104.` |
| 19 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -2.diff_offset ≤ 15<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {), ,, ;}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.length ≤ 13<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.963. Support: 3429.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.473684210526315, "max_conf": 0.9997487664222717, "max_support": 5968, "min_conf": 0.9460573792457581, "min_support": 104, "num_rules": 19}}
```
</details>
