# Model report for file:///tmp/top-repos-quality-repos-0ypvuiim/tech-interview-handbook.git HEAD a5abb22e662530cccbad3932ca7cf93f8a45f7b1

### Dump

```json
{'created_at': '2021-08-23 04:55:23',
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
 'size': '16.0 kB',
 'tags': [],
 'uuid': '55e7e879-15c1-4ace-bc13-92f950c6c45d',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-0ypvuiim/tech-interview-handbook.git a5abb22e662530cccbad3932ca7cf93f8a45f7b1

# javascript
33 rules, avg.len. 3.1
## train
PPCR: 0.874155
### report
macro
{'f1-score': 0.46145686156358207,
 'precision': 0.48192448301811813,
 'recall': 0.46560797349470057,
 'support': 4265}
micro
{'f1-score': 0.8021101992966002,
 'precision': 0.8021101992966002,
 'recall': 0.8021101992966002,
 'support': 4265}
weighted
{'f1-score': 0.7533751146656056,
 'precision': 0.738563588122557,
 'recall': 0.8021101992966002,
 'support': 4265}
### report_full
macro
{'f1-score': 0.4230724211799014,
 'precision': 0.48192448301811813,
 'recall': 0.41363656382884845,
 'support': 4879}
micro
{'f1-score': 0.7482502187226597,
 'precision': 0.8021101992966002,
 'recall': 0.7011682721869236,
 'support': 4879}
weighted
{'f1-score': 0.6836991177331022,
 'precision': 0.7354240813927974,
 'recall': 0.7011682721869236,
 'support': 4879}
## test
PPCR: 0.646478
### report
macro
{'f1-score': 0.14123120512572845,
 'precision': 0.1353527870381803,
 'recall': 0.1990920570980994,
 'support': 1441}
micro
{'f1-score': 0.6127689104788342,
 'precision': 0.6127689104788342,
 'recall': 0.6127689104788342,
 'support': 1441}
weighted
{'f1-score': 0.6093448458854215,
 'precision': 0.6127983004215958,
 'recall': 0.6127689104788342,
 'support': 1441}
### report_full
macro
{'f1-score': 0.11530336905404363,
 'precision': 0.1353527870381803,
 'recall': 0.10635408178190099,
 'support': 2229}
micro
{'f1-score': 0.48119891008174387,
 'precision': 0.6127689104788342,
 'recall': 0.39614176760879316,
 'support': 2229}
weighted
{'f1-score': 0.4653113293176804,
 'precision': 0.5679244854619512,
 'recall': 0.39614176760879316,
 'support': 2229}
```

## javascript
### Summary
9 rules, avg.len. 3.3

| | |
|-|-|
|Min support|239|
|Max support|1159|
|Min confidence|0.9433333277702332|
|Max confidence|0.9983870983123779|

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
                     'max_features': 'auto',
                     'min_samples_leaf': 102,
                     'min_samples_split': 181,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.roles in {LITERAL}<br>	∧ -3.diff_offset ≥ 8<br>	∧ -3.length ≤ 1<br>	∧ +5.roles not in {STRING}<br>⇒ y = "<br>Confidence: 0.943. Support: 750.` |
| 2 | `  -2.diff_offset ≥ 6<br>	∧ ^2.roles in {LIST}<br>⇒ y = "<br>Confidence: 0.979. Support: 651.` |
| 3 | `  -1.reserved = ,<br>	∧ -3.diff_offset ≥ 8<br>	∧ +2.roles not in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.953. Support: 308.` |
| 4 | `  -1.diff_col ≤ 2<br>	∧ -4.diff_col ≤ 9<br>	∧ -5.diff_line = 0<br>	∧ +5.roles in {VALUE}<br>	∧ ^1.roles in {MAP}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 310.` |
| 5 | `  -4.label in {<newline>}<br>	∧ -4.roles not in {STRING}<br>	∧ +1.roles in {MAP}<br>	∧ +2.reserved not in {:}<br>⇒ y = "<br>Confidence: 0.949. Support: 380.` |
| 6 | `  -4.label not in {<newline>}<br>	∧ -4.roles not in {STRING}<br>	∧ +1.roles in {MAP}<br>	∧ +2.reserved not in {:}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 387.` |
| 7 | `  -1.reserved = :<br>⇒ y = ␣<br>Confidence: 0.993. Support: 503.` |
| 8 | `  -1.reserved not in {,}<br>	∧ -3.diff_offset ≥ 6<br>	∧ -4.reserved = "<br>	∧ +3.roles in {KEY}<br>	∧ ^2.internal_type not in {ArrayExpression}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 239.` |
| 9 | `  •••start_line ≥ 70<br>	∧ -2.diff_offset ≥ 5<br>⇒ y = "<br>Confidence: 0.943. Support: 1159.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 3.3333333333333335, "max_conf": 0.9983870983123779, "max_support": 1159, "min_conf": 0.9433333277702332, "min_support": 239, "num_rules": 9}}
```
</details>
