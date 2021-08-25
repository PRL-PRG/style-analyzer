# Model report for file:///tmp/top-repos-quality-repos-gqm2jq0w/node-irsdk.git HEAD 986a9e079cb8b8b780f8d55f1f0988c85596857a

### Dump

```json
{'created_at': '2021-08-24 16:22:54',
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
 'uuid': 'ad3d2afc-da49-4269-a04e-e76eab1f97d2',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-gqm2jq0w/node-irsdk.git 986a9e079cb8b8b780f8d55f1f0988c85596857a

# javascript
11 rules, avg.len. 4.5
## train
PPCR: 0.850191
### report
macro
{'f1-score': 0.7826092803065158,
 'precision': 0.7752635480463733,
 'recall': 0.7923678426708717,
 'support': 5783}
micro
{'f1-score': 0.939304859069687,
 'precision': 0.939304859069687,
 'recall': 0.939304859069687,
 'support': 5783}
weighted
{'f1-score': 0.9362226402876245,
 'precision': 0.9343542105945012,
 'recall': 0.939304859069687,
 'support': 5783}
### report_full
macro
{'f1-score': 0.7101043389814345,
 'precision': 0.7752635480463733,
 'recall': 0.656854818311628,
 'support': 6802}
micro
{'f1-score': 0.8632499006754071,
 'precision': 0.939304859069687,
 'recall': 0.798588650396942,
 'support': 6802}
weighted
{'f1-score': 0.8561105563749283,
 'precision': 0.9246295391989555,
 'recall': 0.798588650396942,
 'support': 6802}
## test
PPCR: 0.496144
### report
macro
{'f1-score': 0.7254006770210022,
 'precision': 0.7122145379214546,
 'recall': 0.7981985880139507,
 'support': 579}
micro
{'f1-score': 0.8721934369602763,
 'precision': 0.8721934369602763,
 'recall': 0.8721934369602763,
 'support': 579}
weighted
{'f1-score': 0.8683867655725929,
 'precision': 0.8751448705313608,
 'recall': 0.8721934369602763,
 'support': 579}
### report_full
macro
{'f1-score': 0.515657162874453,
 'precision': 0.7122145379214546,
 'recall': 0.4247117902693551,
 'support': 1167}
micro
{'f1-score': 0.5784650630011454,
 'precision': 0.8721934369602763,
 'recall': 0.43273350471293914,
 'support': 1167}
weighted
{'f1-score': 0.5599811352537575,
 'precision': 0.815549215046983,
 'recall': 0.4327335047129391,
 'support': 1167}
```

## javascript
### Summary
8 rules, avg.len. 3.9

| | |
|-|-|
|Min support|94|
|Max support|1259|
|Min confidence|0.9201388955116272|
|Max confidence|0.9989407062530518|

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
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.996. Support: 1259.` |
| 2 | `  +2.roles in {ARGUMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.931. Support: 528.` |
| 3 | `  -1.internal_type = StringLiteral<br>	∧ +1.reserved = )<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.995. Support: 100.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +2.roles not in {ARGUMENT}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 472.` |
| 5 | `  -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {)}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.996. Support: 134.` |
| 6 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {)}<br>	∧ +2.roles not in {ARGUMENT}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 319.` |
| 7 | `  -1.reserved not in {(, {}<br>	∧ -1.roles in {LITERAL}<br>	∧ +1.reserved not in {)}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = '<br>Confidence: 0.920. Support: 94.` |
| 8 | `  -1.reserved not in {(}<br>	∧ +1.reserved = }<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.920. Support: 144.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 3.875, "max_conf": 0.9989407062530518, "max_support": 1259, "min_conf": 0.9201388955116272, "min_support": 94, "num_rules": 8}}
```
</details>
