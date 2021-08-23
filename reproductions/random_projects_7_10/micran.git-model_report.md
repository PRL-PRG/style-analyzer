# Model report for file:///tmp/top-repos-quality-repos-n93mvyi0/micran.git HEAD 4c7ac3b5bba8bf184dc99348195a31751b39e714

### Dump

```json
{'created_at': '2021-08-21 05:59:09',
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
 'size': '17.5 kB',
 'tags': [],
 'uuid': '94a11042-cc85-4b3d-be17-758101b7a5ac',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-n93mvyi0/micran.git 4c7ac3b5bba8bf184dc99348195a31751b39e714

# javascript
20 rules, avg.len. 6.7
## train
PPCR: 0.801968
### report
macro
{'f1-score': 0.7374876177141998,
 'precision': 0.8098398185591681,
 'recall': 0.6891361502388234,
 'support': 22083}
micro
{'f1-score': 0.945342571208622,
 'precision': 0.945342571208622,
 'recall': 0.945342571208622,
 'support': 22083}
weighted
{'f1-score': 0.9423515934972279,
 'precision': 0.9434497599004983,
 'recall': 0.945342571208622,
 'support': 22083}
### report_full
macro
{'f1-score': 0.5567197455397047,
 'precision': 0.8098398185591681,
 'recall': 0.45471910715069147,
 'support': 27536}
micro
{'f1-score': 0.8414518631975654,
 'precision': 0.945342571208622,
 'recall': 0.7581348053457292,
 'support': 27536}
weighted
{'f1-score': 0.8193609601904528,
 'precision': 0.937285871134113,
 'recall': 0.7581348053457292,
 'support': 27536}
## test
PPCR: 0.740162
### report
macro
{'f1-score': 0.7495244478705478,
 'precision': 0.826951146186242,
 'recall': 0.7069732640150632,
 'support': 5116}
micro
{'f1-score': 0.9470289288506646,
 'precision': 0.9470289288506646,
 'recall': 0.9470289288506646,
 'support': 5116}
weighted
{'f1-score': 0.944393332392072,
 'precision': 0.947265000994181,
 'recall': 0.9470289288506646,
 'support': 5116}
### report_full
macro
{'f1-score': 0.5462742142667237,
 'precision': 0.826951146186242,
 'recall': 0.441131752240612,
 'support': 6912}
micro
{'f1-score': 0.8056202194878617,
 'precision': 0.9470289288506646,
 'recall': 0.7009548611111112,
 'support': 6912}
weighted
{'f1-score': 0.7801975830677457,
 'precision': 0.9424266471469119,
 'recall': 0.7009548611111112,
 'support': 6912}
```

## javascript
### Summary
9 rules, avg.len. 6.1

| | |
|-|-|
|Min support|98|
|Max support|7831|
|Min confidence|0.9318181872367859|
|Max confidence|0.9967319965362549|

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
| 1 | `  -2.internal_type = JSXIdentifier<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.956. Support: 603.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ -3.roles in {INCOMPLETE}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = "<br>Confidence: 0.953. Support: 579.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved = =<br>	∧ +3.reserved = ><br>⇒ y = ∅<br>Confidence: 0.973. Support: 202.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {DECLARATION, FUNCTION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 153.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 7831.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ +4.roles not in {FUNCTION}<br>	∧ ^1.roles in {QUALIFIED} and not in {DECLARATION, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 2594.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 187.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = import<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type = File<br>⇒ y = ⏎<br>Confidence: 0.974. Support: 98.` |
| 9 | `  -1.internal_type = CommentLine<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {File}<br>⇒ y = ⏎<br>Confidence: 0.932. Support: 110.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.111111111111111, "max_conf": 0.9967319965362549, "max_support": 7831, "min_conf": 0.9318181872367859, "min_support": 98, "num_rules": 9}}
```
</details>
