# Model report for file:///tmp/top-repos-quality-repos-xl1fos8m/workspace.git HEAD 8725ce0589c1b687d498be9f869abf71805d4c67

### Dump

```json
{'created_at': '2021-08-21 21:07:35',
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
 'size': '15.3 kB',
 'tags': [],
 'uuid': 'f4375143-bed7-41ae-8bcb-a345afbab102',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-xl1fos8m/workspace.git 8725ce0589c1b687d498be9f869abf71805d4c67

# javascript
10 rules, avg.len. 4.5
## train
PPCR: 0.684691
### report
macro
{'f1-score': 0.1268334039847393,
 'precision': 0.11404177466077146,
 'recall': 0.14285714285714285,
 'support': 1874}
micro
{'f1-score': 0.7982924226254001,
 'precision': 0.7982924226254002,
 'recall': 0.7982924226254002,
 'support': 1874}
weighted
{'f1-score': 0.7087510173576254,
 'precision': 0.6372707920211306,
 'recall': 0.7982924226254002,
 'support': 1874}
### report_full
macro
{'f1-score': 0.12356998306694751,
 'precision': 0.11404177466077146,
 'recall': 0.1348355114916629,
 'support': 2737}
micro
{'f1-score': 0.6488831056170029,
 'precision': 0.7982924226254002,
 'recall': 0.546583850931677,
 'support': 2737}
weighted
{'f1-score': 0.5009166832765007,
 'precision': 0.4622921044432807,
 'recall': 0.546583850931677,
 'support': 2737}
## test
PPCR: 0.668657
### report
macro
{'f1-score': 0.12338074100914939,
 'precision': 0.10857780612244898,
 'recall': 0.14285714285714285,
 'support': 1792}
micro
{'f1-score': 0.760044642857143,
 'precision': 0.7600446428571429,
 'recall': 0.7600446428571429,
 'support': 1792}
weighted
{'f1-score': 0.6564240986502401,
 'precision': 0.5776678591358418,
 'recall': 0.7600446428571429,
 'support': 1792}
### report_full
macro
{'f1-score': 0.12014290124818065,
 'precision': 0.10857780612244898,
 'recall': 0.1344653963866127,
 'support': 2680}
micro
{'f1-score': 0.60912343470483,
 'precision': 0.7600446428571429,
 'recall': 0.5082089552238805,
 'support': 2680}
weighted
{'f1-score': 0.45407740550105297,
 'precision': 0.41036738739339024,
 'recall': 0.5082089552238805,
 'support': 2680}
```

## javascript
### Summary
4 rules, avg.len. 4.0

| | |
|-|-|
|Min support|185|
|Max support|622|
|Min confidence|0.9417744874954224|
|Max confidence|0.9594594836235046|

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
                     'min_samples_split': 182,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {CALL}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 542.` |
| 2 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>⇒ y = ∅<br>Confidence: 0.959. Support: 185.` |
| 3 | `  -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {EXPRESSION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.943. Support: 622.` |
| 4 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {CALL}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 541.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.0, "max_conf": 0.9594594836235046, "max_support": 622, "min_conf": 0.9417744874954224, "min_support": 185, "num_rules": 4}}
```
</details>