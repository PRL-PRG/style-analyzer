# Model report for file:///tmp/top-repos-quality-repos-kkyut8m1/enquire.js.git HEAD 2f339f1e29b2b6676f541e64b770635075af494d

### Dump

```json
{'created_at': '2021-08-30 03:39:52',
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
 'size': '13.6 kB',
 'tags': [],
 'uuid': 'abd9d302-a41d-425a-8828-7611a4178e23',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-kkyut8m1/enquire.js.git 2f339f1e29b2b6676f541e64b770635075af494d

# javascript
5 rules, avg.len. 5.4
## train
PPCR: 0.730920
### report
macro
{'f1-score': 0.5551552842873616,
 'precision': 0.5769712964855285,
 'recall': 0.540137504350037,
 'support': 2423}
micro
{'f1-score': 0.9401568303755675,
 'precision': 0.9401568303755675,
 'recall': 0.9401568303755675,
 'support': 2423}
weighted
{'f1-score': 0.9330725133718796,
 'precision': 0.9324386220286187,
 'recall': 0.9401568303755675,
 'support': 2423}
### report_full
macro
{'f1-score': 0.4266180527348202,
 'precision': 0.5769712964855285,
 'recall': 0.3621184425297689,
 'support': 3315}
micro
{'f1-score': 0.7940048797490414,
 'precision': 0.9401568303755675,
 'recall': 0.6871794871794872,
 'support': 3315}
weighted
{'f1-score': 0.7368147151992184,
 'precision': 0.8624075015776363,
 'recall': 0.6871794871794872,
 'support': 3315}
## test
PPCR: 0.708791
### report
macro
{'f1-score': 0.364782029316581,
 'precision': 0.3850467289719626,
 'recall': 0.35172413793103446,
 'support': 129}
micro
{'f1-score': 0.9379844961240311,
 'precision': 0.937984496124031,
 'recall': 0.937984496124031,
 'support': 129}
weighted
{'f1-score': 0.9315887414648256,
 'precision': 0.9348692313265232,
 'recall': 0.937984496124031,
 'support': 129}
### report_full
macro
{'f1-score': 0.303536231884058,
 'precision': 0.3850467289719626,
 'recall': 0.26141363144608726,
 'support': 182}
micro
{'f1-score': 0.7781350482315113,
 'precision': 0.937984496124031,
 'recall': 0.6648351648351648,
 'support': 182}
weighted
{'f1-score': 0.7352253543557892,
 'precision': 0.8581185169970217,
 'recall': 0.6648351648351648,
 'support': 182}
```

## javascript
### Summary
4 rules, avg.len. 5.5

| | |
|-|-|
|Min support|125|
|Max support|1347|
|Min confidence|0.9303797483444214|
|Max confidence|0.9962406158447266|

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
                     'max_depth': 10,
                     'min_samples_leaf': 97,
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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.996. Support: 133.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.roles not in {STRING}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.988. Support: 125.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 2<br>	∧ ^1.internal_type not in {ObjectProperty}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.930. Support: 237.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -3.diff_line = 0<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {ObjectProperty}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.937. Support: 1347.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.5, "max_conf": 0.9962406158447266, "max_support": 1347, "min_conf": 0.9303797483444214, "min_support": 125, "num_rules": 4}}
```
</details>
