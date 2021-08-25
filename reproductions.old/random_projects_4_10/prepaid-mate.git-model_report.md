# Model report for file:///tmp/top-repos-quality-repos-zk_qm53x/prepaid-mate.git HEAD 17bf38f67a33d0ce5f099e4a0e02f09d7fdfaa9c

### Dump

```json
{'created_at': '2021-08-21 23:01:57',
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
 'size': '16.7 kB',
 'tags': [],
 'uuid': '3be57dd5-1578-41aa-bff4-2f342fb13470',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-zk_qm53x/prepaid-mate.git 17bf38f67a33d0ce5f099e4a0e02f09d7fdfaa9c

# javascript
28 rules, avg.len. 8.5
## train
PPCR: 0.933829
### report
macro
{'f1-score': 0.8252457315867234,
 'precision': 0.829958181889228,
 'recall': 0.8220121049645936,
 'support': 31781}
micro
{'f1-score': 0.9748591925993518,
 'precision': 0.9748591925993518,
 'recall': 0.9748591925993518,
 'support': 31781}
weighted
{'f1-score': 0.9722522951522401,
 'precision': 0.9700467805915496,
 'recall': 0.9748591925993518,
 'support': 31781}
### report_full
macro
{'f1-score': 0.7355724049348658,
 'precision': 0.829958181889228,
 'recall': 0.6940233280487385,
 'support': 34033}
micro
{'f1-score': 0.9415018081259308,
 'precision': 0.9748591925993518,
 'recall': 0.9103517174507096,
 'support': 34033}
weighted
{'f1-score': 0.9294171492442705,
 'precision': 0.9623296114826492,
 'recall': 0.9103517174507096,
 'support': 34033}
## test
PPCR: 0.921746
### report
macro
{'f1-score': 0.567045301842948,
 'precision': 0.5940869431570983,
 'recall': 0.5491263092707384,
 'support': 1119}
micro
{'f1-score': 0.9016979445933869,
 'precision': 0.9016979445933869,
 'recall': 0.9016979445933869,
 'support': 1119}
weighted
{'f1-score': 0.8990573400640942,
 'precision': 0.9062306047567332,
 'recall': 0.9016979445933869,
 'support': 1119}
### report_full
macro
{'f1-score': 0.39988547020208226,
 'precision': 0.5940869431570983,
 'recall': 0.3561432474470825,
 'support': 1214}
micro
{'f1-score': 0.8649807115302186,
 'precision': 0.9016979445933869,
 'recall': 0.8311367380560132,
 'support': 1214}
weighted
{'f1-score': 0.8481603012398987,
 'precision': 0.9111816902990755,
 'recall': 0.8311367380560132,
 'support': 1214}
```

## javascript
### Summary
19 rules, avg.len. 8.2

| | |
|-|-|
|Min support|94|
|Max support|6216|
|Min confidence|0.9214876294136047|
|Max confidence|0.9997344613075256|

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
| 1 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 6216.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 105.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.reserved not in {]}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.946. Support: 2865.` |
| 4 | `  -1.reserved = {<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.992. Support: 812.` |
| 5 | `  -1.reserved not in {{}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.954. Support: 850.` |
| 6 | `  -1.reserved not in {{}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.948. Support: 1267.` |
| 7 | `  -1.internal_type = Identifier<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 3684.` |
| 8 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1883.` |
| 9 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.931. Support: 94.` |
| 10 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 1248.` |
| 11 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved not in {;, if, }}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles in {CALLEE}<br>⇒ y = ⏎⏎<br>Confidence: 0.964. Support: 96.` |
| 12 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved not in {;, if, }}<br>	∧ +4.reserved = =<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {SCOPE} and not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles not in {CALLEE}<br>⇒ y = ⏎<br>Confidence: 0.921. Support: 121.` |
| 13 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 391.` |
| 14 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label in {<newline>}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.987. Support: 198.` |
| 15 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_col ≥ 113<br>	∧ -2.label not in {<newline>}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 117.` |
| 16 | `  -1.internal_type not in {Identifier}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {File, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 295.` |
| 17 | `  -1.internal_type not in {Identifier}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.internal_type not in {File, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.986. Support: 259.` |
| 18 | `  -1.internal_type not in {Identifier}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 169.` |
| 19 | `  -1.internal_type not in {Identifier}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {STRING}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 3118.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.157894736842104, "max_conf": 0.9997344613075256, "max_support": 6216, "min_conf": 0.9214876294136047, "min_support": 94, "num_rules": 19}}
```
</details>
