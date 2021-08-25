# Model report for file:///tmp/top-repos-quality-repos-2onr0x6i/practice_test.git HEAD f604ccc8e1cccd7460980cdcf89f8f8c0f0c4241

### Dump

```json
{'created_at': '2021-08-21 22:48:37',
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
 'size': '15.1 kB',
 'tags': [],
 'uuid': 'c5f3b61c-44e7-416d-913f-a85bf18a6e49',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-2onr0x6i/practice_test.git f604ccc8e1cccd7460980cdcf89f8f8c0f0c4241

# javascript
26 rules, avg.len. 4.8
## train
PPCR: 0.812940
### report
macro
{'f1-score': 0.4425452344401908,
 'precision': 0.4372189061775677,
 'recall': 0.4486517261777341,
 'support': 4850}
micro
{'f1-score': 0.8756701030927835,
 'precision': 0.8756701030927835,
 'recall': 0.8756701030927835,
 'support': 4850}
weighted
{'f1-score': 0.8451138849095955,
 'precision': 0.8173796147622143,
 'recall': 0.8756701030927835,
 'support': 4850}
### report_full
macro
{'f1-score': 0.3733222431705288,
 'precision': 0.4372189061775677,
 'recall': 0.3333530911417375,
 'support': 5966}
micro
{'f1-score': 0.7853180473372781,
 'precision': 0.8756701030927835,
 'recall': 0.7118672477371774,
 'support': 5966}
weighted
{'f1-score': 0.7488141257256578,
 'precision': 0.8033958438671078,
 'recall': 0.7118672477371774,
 'support': 5966}
## test
PPCR: 0.677609
### report
macro
{'f1-score': 0.4268961990348177,
 'precision': 0.4405341184274492,
 'recall': 0.41873778207975104,
 'support': 805}
micro
{'f1-score': 0.8645962732919255,
 'precision': 0.8645962732919255,
 'recall': 0.8645962732919255,
 'support': 805}
weighted
{'f1-score': 0.8239011576470148,
 'precision': 0.7919709333893891,
 'recall': 0.8645962732919255,
 'support': 805}
### report_full
macro
{'f1-score': 0.3047265213428511,
 'precision': 0.4405341184274492,
 'recall': 0.24917905171969723,
 'support': 1188}
micro
{'f1-score': 0.6984445559458103,
 'precision': 0.8645962732919255,
 'recall': 0.5858585858585859,
 'support': 1188}
weighted
{'f1-score': 0.647232578675335,
 'precision': 0.7765553649176394,
 'recall': 0.5858585858585859,
 'support': 1188}
```

## javascript
### Summary
14 rules, avg.len. 5.3

| | |
|-|-|
|Min support|146|
|Max support|1049|
|Min confidence|0.9335260391235352|
|Max confidence|0.9987775087356567|

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
                     'max_depth': 10,
                     'min_samples_leaf': 90,
                     'min_samples_split': 185,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.internal_type = VariableDeclarator<br>⇒ y = ␣<br>Confidence: 0.998. Support: 268.` |
| 2 | `  -1.diff_col ≥ 3<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<newline>}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 156.` |
| 3 | `  -1.diff_col ≤ 2<br>	∧ -1.reserved = .<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 409.` |
| 4 | `  -1.reserved not in {;}<br>	∧ ^1.roles not in {CALL}<br>	∧ ^2.roles in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 286.` |
| 5 | `  -1.reserved not in {;}<br>	∧ -2.label in {<newline>}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {CALL, OPERATOR}<br>	∧ ^2.roles not in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.964. Support: 154.` |
| 6 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {CALL, OPERATOR}<br>	∧ ^2.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 172.` |
| 7 | `  -1.reserved not in {;}<br>	∧ ^1.roles not in {CALL}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.994. Support: 253.` |
| 8 | `  -1.diff_col ≥ 3<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<newline>}<br>	∧ ^1.roles not in {CALL, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.934. Support: 173.` |
| 9 | `  -1.diff_col ≤ 2<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {ObjectProperty}<br>	∧ ^1.roles not in {CALL, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 146.` |
| 10 | `  ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.991. Support: 277.` |
| 11 | `  -1.internal_type = Identifier<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 1049.` |
| 12 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {CALL, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 183.` |
| 13 | `  -1.reserved not in {;}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 279.` |
| 14 | `  -1.diff_offset ≤ 2<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {CALL, OPERATOR}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 171.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.285714285714286, "max_conf": 0.9987775087356567, "max_support": 1049, "min_conf": 0.9335260391235352, "min_support": 146, "num_rules": 14}}
```
</details>
