# Model report for file:///tmp/top-repos-quality-repos-errj3mnf/sinon-chai.git HEAD 74a1bc0d5b9518341f255bf771c15bddff97664e

### Dump

```json
{'created_at': '2021-08-30 08:31:04',
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
 'size': '16.6 kB',
 'tags': [],
 'uuid': '64f8f165-afb3-4d07-83c5-961230502890',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-errj3mnf/sinon-chai.git 74a1bc0d5b9518341f255bf771c15bddff97664e

# javascript
43 rules, avg.len. 5.9
## train
PPCR: 0.973531
### report
macro
{'f1-score': 0.8129444371040363,
 'precision': 0.8161512678621357,
 'recall': 0.81204157459885,
 'support': 14087}
micro
{'f1-score': 0.9681976290196636,
 'precision': 0.9681976290196636,
 'recall': 0.9681976290196636,
 'support': 14087}
weighted
{'f1-score': 0.965535109674821,
 'precision': 0.9649329293010741,
 'recall': 0.9681976290196636,
 'support': 14087}
### report_full
macro
{'f1-score': 0.7853693714415689,
 'precision': 0.8161512678621357,
 'recall': 0.7630523071586334,
 'support': 14470}
micro
{'f1-score': 0.9552123822530377,
 'precision': 0.9681976290196636,
 'recall': 0.9425708362128542,
 'support': 14470}
weighted
{'f1-score': 0.9432454671625602,
 'precision': 0.9470257694888998,
 'recall': 0.9425708362128542,
 'support': 14470}
## test
PPCR: 0.971986
### report
macro
{'f1-score': 0.8127144829818989,
 'precision': 0.8316154140759746,
 'recall': 0.8004187851333443,
 'support': 1943}
micro
{'f1-score': 0.9500772002058672,
 'precision': 0.9500772002058672,
 'recall': 0.9500772002058672,
 'support': 1943}
weighted
{'f1-score': 0.9466948324844722,
 'precision': 0.9507173676637628,
 'recall': 0.9500772002058672,
 'support': 1943}
### report_full
macro
{'f1-score': 0.7809913395067726,
 'precision': 0.8316154140759746,
 'recall': 0.7452809405218154,
 'support': 1999}
micro
{'f1-score': 0.9365804160324707,
 'precision': 0.9500772002058672,
 'recall': 0.9234617308654327,
 'support': 1999}
weighted
{'f1-score': 0.9243954055436283,
 'precision': 0.9342206119386833,
 'recall': 0.9234617308654327,
 'support': 1999}
```

## javascript
### Summary
36 rules, avg.len. 5.9

| | |
|-|-|
|Min support|150|
|Max support|8157|
|Min confidence|0.9251968264579773|
|Max confidence|0.9987277388572693|

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
                     'base_model_name': 'sklearn.ensemble.RandomForestClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 90,
                     'min_samples_split': 239,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.reserved = ;<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.999. Support: 393.` |
| 2 | `  -1.reserved = {<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.995. Support: 490.` |
| 3 | `  -1.reserved not in {;, {}<br>	∧ +1.reserved = {<br>⇒ y = ␣<br>Confidence: 0.992. Support: 465.` |
| 4 | `  -1.reserved = function<br>⇒ y = ␣<br>Confidence: 0.989. Support: 421.` |
| 5 | `  -1.reserved = ,<br>⇒ y = ␣<br>Confidence: 0.978. Support: 393.` |
| 6 | `  -1.reserved not in {,, ;, function, {}<br>	∧ -1.roles in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>⇒ y = "<br>Confidence: 0.998. Support: 324.` |
| 7 | `  -1.reserved not in {;, function, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = VariableDeclarator<br>⇒ y = ␣<br>Confidence: 0.992. Support: 186.` |
| 8 | `  -1.reserved not in {,, ;, function, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 8054.` |
| 9 | `  -1.reserved not in {,, ;, function, {}<br>	∧ -1.roles in {STRING}<br>	∧ +1.reserved not in {{}<br>⇒ y = "<br>Confidence: 0.998. Support: 330.` |
| 10 | `  -1.reserved not in {;, function, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.984. Support: 160.` |
| 11 | `  -1.reserved not in {,, ;, function, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 8044.` |
| 12 | `  -1.reserved not in {;}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 4165.` |
| 13 | `  -1.reserved = {<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.989. Support: 407.` |
| 14 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {;, {}<br>	∧ -4.length ≥ 8<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.997. Support: 150.` |
| 15 | `  -1.reserved = ,<br>	∧ -4.length ≤ 7<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.944. Support: 257.` |
| 16 | `  -1.reserved = function<br>	∧ -4.length ≤ 7<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 194.` |
| 17 | `  -1.reserved not in {,, ;, function, {}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -3.reserved = .<br>	∧ -4.length ≤ 7<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 369.` |
| 18 | `  -1.reserved not in {,, ;, function, {}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -4.length ≤ 7<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 3039.` |
| 19 | `  -1.reserved not in {,, ;, function, {}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles in {STRING}<br>⇒ y = "<br>Confidence: 0.925. Support: 381.` |
| 20 | `  -1.reserved not in {,, ;, function, {}<br>	∧ -1.roles in {STRING}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {STRING}<br>⇒ y = "<br>Confidence: 0.998. Support: 321.` |
| 21 | `  -1.reserved not in {;, function, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type = VariableDeclarator<br>⇒ y = ␣<br>Confidence: 0.985. Support: 162.` |
| 22 | `  -1.reserved not in {,, ;, function, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 8142.` |
| 23 | `  -1.reserved not in {;, function, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.984. Support: 156.` |
| 24 | `  -1.reserved not in {,, ;, function, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {STRING}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 8091.` |
| 25 | `  -1.reserved not in {;, function, {}<br>	∧ +1.reserved = {<br>⇒ y = ␣<br>Confidence: 0.987. Support: 410.` |
| 26 | `  -1.reserved not in {,, ;, function, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ +3.length ≤ 2<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>⇒ y = ∅<br>Confidence: 0.944. Support: 1205.` |
| 27 | `  -1.reserved not in {,, ;, function, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 6812.` |
| 28 | `  •••start_col ≤ 17<br>	∧ -1.reserved = ;<br>	∧ +1.reserved not in {}}<br>⇒ y = ⏎⏎<br>Confidence: 0.944. Support: 153.` |
| 29 | `  -1.reserved not in {;, function, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 173.` |
| 30 | `  -1.reserved not in {,, ;, function, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 8131.` |
| 31 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {,, ;, function, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>⇒ y = "<br>Confidence: 0.998. Support: 291.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, function, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 161.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, function, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ ^1.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 8157.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, function, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.991. Support: 160.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, function, {}<br>	∧ -2.diff_col ≥ 17<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ +1.length ≤ 1<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 746.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, function, {}<br>	∧ -2.diff_col ≤ 16<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 7159.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.944444444444445, "max_conf": 0.9987277388572693, "max_support": 8157, "min_conf": 0.9251968264579773, "min_support": 150, "num_rules": 36}}
```
</details>
