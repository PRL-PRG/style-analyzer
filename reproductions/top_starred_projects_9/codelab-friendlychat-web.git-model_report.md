# Model report for file:///tmp/top-repos-quality-repos-ajcn6pfw/codelab-friendlychat-web.git HEAD 1bb39a63bbbafccbe22b647e9660c88b305347d7

### Dump

```json
{'created_at': '2021-08-29 11:52:44',
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
 'size': '16.2 kB',
 'tags': [],
 'uuid': 'a9ad7ba1-610a-4b73-83ac-67cb98bebf3b',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ajcn6pfw/codelab-friendlychat-web.git 1bb39a63bbbafccbe22b647e9660c88b305347d7

# javascript
15 rules, avg.len. 5.7
## train
PPCR: 0.929260
### report
macro
{'f1-score': 0.7868260082390827,
 'precision': 0.7918079993263171,
 'recall': 0.7826818391024555,
 'support': 11560}
micro
{'f1-score': 0.946885813148789,
 'precision': 0.946885813148789,
 'recall': 0.946885813148789,
 'support': 11560}
weighted
{'f1-score': 0.9438129395787987,
 'precision': 0.9414603290015875,
 'recall': 0.946885813148789,
 'support': 11560}
### report_full
macro
{'f1-score': 0.765598530582733,
 'precision': 0.7918079993263171,
 'recall': 0.7422051142202586,
 'support': 12440}
micro
{'f1-score': 0.9121666666666666,
 'precision': 0.946885813148789,
 'recall': 0.8799035369774919,
 'support': 12440}
weighted
{'f1-score': 0.899074243148814,
 'precision': 0.9201617003423467,
 'recall': 0.8799035369774919,
 'support': 12440}
## test
PPCR: 0.923353
### report
macro
{'f1-score': 0.7440287956213657,
 'precision': 0.7418111257349758,
 'recall': 0.7490831403870695,
 'support': 1542}
micro
{'f1-score': 0.8994811932555123,
 'precision': 0.8994811932555123,
 'recall': 0.8994811932555123,
 'support': 1542}
weighted
{'f1-score': 0.8939480245775868,
 'precision': 0.8901783621760389,
 'recall': 0.8994811932555123,
 'support': 1542}
### report_full
macro
{'f1-score': 0.7136294095568142,
 'precision': 0.7418111257349758,
 'recall': 0.6914013920755288,
 'support': 1670}
micro
{'f1-score': 0.8636363636363636,
 'precision': 0.8994811932555123,
 'recall': 0.8305389221556886,
 'support': 1670}
weighted
{'f1-score': 0.8535593054994066,
 'precision': 0.8802354246907826,
 'recall': 0.8305389221556886,
 'support': 1670}
```

## javascript
### Summary
11 rules, avg.len. 5.3

| | |
|-|-|
|Min support|117|
|Max support|2267|
|Min confidence|0.9279661178588867|
|Max confidence|0.9988687634468079|

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
                     'min_samples_split': 187,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 2090.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.999. Support: 442.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 401.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.995. Support: 276.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>⇒ y = ∅<br>Confidence: 0.996. Support: 356.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -2.diff_offset ≤ 15<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 3<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.939. Support: 271.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.diff_offset ≤ 15<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.996. Support: 117.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.928. Support: 354.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 279.` |
| 10 | `  •••start_col ≥ 11<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.963. Support: 2267.` |
| 11 | `  •••start_col ≤ 10<br>	∧ -1.diff_col ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 130.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.2727272727272725, "max_conf": 0.9988687634468079, "max_support": 2267, "min_conf": 0.9279661178588867, "min_support": 117, "num_rules": 11}}
```
</details>
