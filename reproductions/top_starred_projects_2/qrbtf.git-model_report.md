# Model report for file:///tmp/top-repos-quality-repos-lhgniwdb/qrbtf.git HEAD 04730a769a5a134e76a114f9f7d61fcf6dc48079

### Dump

```json
{'created_at': '2021-08-30 03:49:56',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.11.0-31-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '18.4 kB',
 'tags': [],
 'uuid': 'a0873af8-3a68-47ad-bf36-f31f8e93df88',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-lhgniwdb/qrbtf.git 04730a769a5a134e76a114f9f7d61fcf6dc48079

# javascript
26 rules, avg.len. 7.2
## train
PPCR: 0.872803
### report
macro
{'f1-score': 0.36150780552044576,
 'precision': 0.37134661925896806,
 'recall': 0.35438653391574426,
 'support': 20908}
micro
{'f1-score': 0.9353357566481729,
 'precision': 0.9353357566481729,
 'recall': 0.9353357566481729,
 'support': 20908}
weighted
{'f1-score': 0.9231252822493862,
 'precision': 0.9120822689680919,
 'recall': 0.9353357566481729,
 'support': 20908}
### report_full
macro
{'f1-score': 0.31502922212214723,
 'precision': 0.37134661925896806,
 'recall': 0.28825204178260966,
 'support': 23955}
micro
{'f1-score': 0.8718097318502999,
 'precision': 0.9353357566481729,
 'recall': 0.8163640158630766,
 'support': 23955}
weighted
{'f1-score': 0.8332644092722885,
 'precision': 0.8631972198997845,
 'recall': 0.8163640158630766,
 'support': 23955}
## test
PPCR: 0.888255
### report
macro
{'f1-score': 0.35126246761640667,
 'precision': 0.36670490074739703,
 'recall': 0.34148562654762327,
 'support': 3895}
micro
{'f1-score': 0.9183568677792041,
 'precision': 0.9183568677792041,
 'recall': 0.9183568677792041,
 'support': 3895}
weighted
{'f1-score': 0.9034615265042223,
 'precision': 0.891601501513971,
 'recall': 0.9183568677792041,
 'support': 3895}
### report_full
macro
{'f1-score': 0.3123583216518858,
 'precision': 0.36670490074739703,
 'recall': 0.28578188042619856,
 'support': 4385}
micro
{'f1-score': 0.8640096618357488,
 'precision': 0.9183568677792041,
 'recall': 0.8157354618015964,
 'support': 4385}
weighted
{'f1-score': 0.8268057704186027,
 'precision': 0.8504363399435757,
 'recall': 0.8157354618015964,
 'support': 4385}
```

## javascript
### Summary
14 rules, avg.len. 7.0

| | |
|-|-|
|Min support|95|
|Max support|4594|
|Min confidence|0.9265129566192627|
|Max confidence|0.9948453903198242|

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
| 1 | `  +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 95.` |
| 2 | `  -1.reserved = ,<br>	∧ -2.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 926.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ ^1.roles in {EXPRESSION} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 4594.` |
| 4 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {,}<br>	∧ +2.reserved = =<br>	∧ ^1.roles in {INCOMPLETE} and not in {EXPRESSION, OPERATOR}<br>⇒ y = "<br>Confidence: 0.961. Support: 115.` |
| 5 | `  -1.reserved not in {,}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE} and not in {EXPRESSION, OPERATOR}<br>⇒ y = "<br>Confidence: 0.974. Support: 249.` |
| 6 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE} and not in {EXPRESSION, OPERATOR}<br>⇒ y = "<br>Confidence: 0.968. Support: 110.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE} and not in {EXPRESSION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 3024.` |
| 8 | `  -1.reserved = (<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {EXPRESSION, INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 170.` |
| 9 | `  -1.diff_col ≥ 2<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, {}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles in {SCOPE} and not in {EXPRESSION, INCOMPLETE, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.957. Support: 151.` |
| 10 | `  -1.reserved not in {,}<br>	∧ -3.length ≥ 3<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {DECLARATION} and not in {EXPRESSION, INCOMPLETE, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.930. Support: 448.` |
| 11 | `  -1.reserved = ;<br>	∧ +1.reserved = import<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION, INCOMPLETE, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.995. Support: 97.` |
| 12 | `  -1.reserved not in {,, ;}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION, INCOMPLETE, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 242.` |
| 13 | `  -1.reserved not in {,, ;}<br>	∧ -1.roles not in {STRING}<br>	∧ -4.diff_line ≥ 1<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION, INCOMPLETE, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.957. Support: 174.` |
| 14 | `  •••start_line ≤ 147<br>	∧ -1.reserved not in {,, ;}<br>	∧ -1.roles not in {STRING}<br>	∧ -4.diff_line ≥ 1<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION, INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.927. Support: 347.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.0, "max_conf": 0.9948453903198242, "max_support": 4594, "min_conf": 0.9265129566192627, "min_support": 95, "num_rules": 14}}
```
</details>