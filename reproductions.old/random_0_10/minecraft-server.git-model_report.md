# Model report for file:///tmp/top-repos-quality-repos-b943es_f/minecraft-server.git HEAD 9c9e2abbe25a4642cb509a8eb80dabb41b5f6968

### Dump

```json
{'created_at': '2021-08-20 00:36:32',
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
 'size': '20.4 kB',
 'tags': [],
 'uuid': 'b45381b0-5d05-4165-b0c9-715ca3d0fcd1',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-b943es_f/minecraft-server.git 9c9e2abbe25a4642cb509a8eb80dabb41b5f6968

# javascript
29 rules, avg.len. 8.1
## train
PPCR: 0.938185
### report
macro
{'f1-score': 0.5626405330743648,
 'precision': 0.5892225319578445,
 'recall': 0.5518213442751757,
 'support': 57158}
micro
{'f1-score': 0.9673536512824101,
 'precision': 0.9673536512824101,
 'recall': 0.9673536512824101,
 'support': 57158}
weighted
{'f1-score': 0.9644128341153585,
 'precision': 0.963078691552853,
 'recall': 0.9673536512824101,
 'support': 57158}
### report_full
macro
{'f1-score': 0.49295659122495206,
 'precision': 0.5892225319578445,
 'recall': 0.4695279107747142,
 'support': 60924}
micro
{'f1-score': 0.9365017530190886,
 'precision': 0.9673536512824101,
 'recall': 0.9075569562077342,
 'support': 60924}
weighted
{'f1-score': 0.9180538945394894,
 'precision': 0.9538833342201507,
 'recall': 0.9075569562077342,
 'support': 60924}
## test
PPCR: 0.935105
### report
macro
{'f1-score': 0.46740620624593576,
 'precision': 0.47947432632097403,
 'recall': 0.5022810510459945,
 'support': 12277}
micro
{'f1-score': 0.8960658141239717,
 'precision': 0.8960658141239717,
 'recall': 0.8960658141239717,
 'support': 12277}
weighted
{'f1-score': 0.8901783625975729,
 'precision': 0.8958699244330126,
 'recall': 0.8960658141239717,
 'support': 12277}
### report_full
macro
{'f1-score': 0.4035197824497773,
 'precision': 0.47947432632097403,
 'recall': 0.4052858660652401,
 'support': 13129}
micro
{'f1-score': 0.8660159017554908,
 'precision': 0.8960658141239717,
 'recall': 0.8379160636758322,
 'support': 13129}
weighted
{'f1-score': 0.8401807766129729,
 'precision': 0.8897471936128697,
 'recall': 0.8379160636758322,
 'support': 13129}
```

## javascript
### Summary
20 rules, avg.len. 7.5

| | |
|-|-|
|Min support|93|
|Max support|12454|
|Min confidence|0.9338017702102661|
|Max confidence|0.9997121691703796|

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
                     'min_samples_split': 240,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.roles in {STRING}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.977. Support: 107.` |
| 2 | `  -1.roles not in {STRING}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 12454.` |
| 3 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 200.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 101.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved not in {), ]}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.944. Support: 4736.` |
| 6 | `  -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 722.` |
| 7 | `  -1.roles in {EXPRESSION, STRING}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = '<br>Confidence: 0.964. Support: 732.` |
| 8 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 7927.` |
| 9 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = '<br>Confidence: 0.948. Support: 415.` |
| 10 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 3094.` |
| 11 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1737.` |
| 12 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.934. Support: 1352.` |
| 13 | `  •••start_col ≥ 5<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ ^1.internal_type = ForStatement<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 137.` |
| 14 | `  •••start_col ≥ 5<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 385.` |
| 15 | `  •••start_col ≥ 5<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ;, }}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = '<br>Confidence: 0.940. Support: 294.` |
| 16 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 265.` |
| 17 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 257.` |
| 18 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 93.` |
| 19 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {COMMENT, EXPRESSION}<br>	∧ -2.reserved = (<br>	∧ -3.reserved not in {,}<br>	∧ -4.diff_offset ≤ 6<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 123.` |
| 20 | `  •••start_col ≥ 5<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {COMMENT, EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 6797.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.45, "max_conf": 0.9997121691703796, "max_support": 12454, "min_conf": 0.9338017702102661, "min_support": 93, "num_rules": 20}}
```
</details>
