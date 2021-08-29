# Model report for file:///tmp/top-repos-quality-repos-d0fkp1fk/web-vitals.git HEAD ad735bf7dbc811f36f9673b25b0f5cd3ab042961

### Dump

```json
{'created_at': '2021-08-29 12:30:33',
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
 'size': '15.7 kB',
 'tags': [],
 'uuid': '006c3f01-55b1-45eb-9947-010dee49bf9b',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-d0fkp1fk/web-vitals.git ad735bf7dbc811f36f9673b25b0f5cd3ab042961

# javascript
10 rules, avg.len. 5.5
## train
PPCR: 0.832068
### report
macro
{'f1-score': 0.6551248830982008,
 'precision': 0.6648228724053009,
 'recall': 0.6522196295433854,
 'support': 6575}
micro
{'f1-score': 0.9376425855513308,
 'precision': 0.9376425855513308,
 'recall': 0.9376425855513308,
 'support': 6575}
weighted
{'f1-score': 0.932812862943258,
 'precision': 0.9348019099681483,
 'recall': 0.9376425855513308,
 'support': 6575}
### report_full
macro
{'f1-score': 0.5456381022485672,
 'precision': 0.6648228724053009,
 'recall': 0.47857974796261427,
 'support': 7902}
micro
{'f1-score': 0.8516957933273469,
 'precision': 0.9376425855513308,
 'recall': 0.7801822323462415,
 'support': 7902}
weighted
{'f1-score': 0.8189699498157359,
 'precision': 0.8942885937719612,
 'recall': 0.7801822323462415,
 'support': 7902}
## test
PPCR: 0.822803
### report
macro
{'f1-score': 0.665524490241632,
 'precision': 0.675672376486764,
 'recall': 0.6619890051572137,
 'support': 1133}
micro
{'f1-score': 0.9443954104148279,
 'precision': 0.9443954104148279,
 'recall': 0.9443954104148279,
 'support': 1133}
weighted
{'f1-score': 0.9415959486283605,
 'precision': 0.946627547684796,
 'recall': 0.9443954104148279,
 'support': 1133}
### report_full
macro
{'f1-score': 0.5653174717814325,
 'precision': 0.675672376486764,
 'recall': 0.5014414019379263,
 'support': 1377}
micro
{'f1-score': 0.8525896414342629,
 'precision': 0.9443954104148279,
 'recall': 0.7770515613652869,
 'support': 1377}
weighted
{'f1-score': 0.8160349835673822,
 'precision': 0.8892468333574942,
 'recall': 0.7770515613652869,
 'support': 1377}
```

## javascript
### Summary
7 rules, avg.len. 4.7

| | |
|-|-|
|Min support|96|
|Max support|3985|
|Min confidence|0.92885822057724|
|Max confidence|0.9977169036865234|

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
                     'min_samples_split': 233,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.reserved = ;<br>	∧ +2.reserved = .<br>⇒ y = ⏎<br>Confidence: 0.942. Support: 113.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {;}<br>⇒ y = '<br>Confidence: 0.998. Support: 219.` |
| 3 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {;}<br>⇒ y = ⏎<br>Confidence: 0.987. Support: 188.` |
| 4 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = await<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 190.` |
| 5 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {CALL}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 140.` |
| 6 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, await}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 96.` |
| 7 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, await, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.929. Support: 3985.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.714285714285714, "max_conf": 0.9977169036865234, "max_support": 3985, "min_conf": 0.92885822057724, "min_support": 96, "num_rules": 7}}
```
</details>
